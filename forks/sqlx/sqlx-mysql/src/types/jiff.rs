use bytes::Buf;
use jiff::civil::{Date, DateTime, Time};
use jiff::tz::Offset;
use jiff::{SignedDuration, Timestamp};
use sqlx_core::database::Database;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::{BoxDynError, UnexpectedNullError};
use crate::protocol::text::ColumnType;
use crate::type_info::MySqlTypeInfo;
use crate::types::{MySqlTime, MySqlTimeSign, Type};
use crate::{MySql, MySqlValueFormat, MySqlValueRef};

impl Type<MySql> for Timestamp {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Timestamp)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(ty.r#type, ColumnType::Datetime | ColumnType::Timestamp)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl Encode<'_, MySql> for Timestamp {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        Encode::<MySql>::encode(Offset::UTC.to_datetime(*self), buf)
    }
}

/// Note: assumes the connection's `time_zone` is set to `+00:00` (UTC).
impl<'r> Decode<'r, MySql> for Timestamp {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let civil: DateTime = Decode::<MySql>::decode(value)?;

        Ok(Offset::UTC.to_timestamp(civil)?)
    }
}

impl Type<MySql> for Time {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl Encode<'_, MySql> for Time {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = time_encoded_len(self);
        buf.push(len);

        // Time is not negative
        buf.push(0);

        // Number of days in the interval; always 0 for time-of-day values.
        // https://mariadb.com/kb/en/resultset-row/#teimstamp-binary-encoding
        buf.extend_from_slice(&[0_u8; 4]);

        encode_time(self, len > 8, buf);

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        time_encoded_len(self) as usize + 1 // plus length byte
    }
}

/// Decode from a `TIME` value.
///
/// ### Errors
/// Returns an error if the `TIME` value is negative or exceeds `23:59:59.999999`.
impl<'r> Decode<'r, MySql> for Time {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                // Covers most possible failure modes.
                MySqlTime::decode(value)?.try_into()
            }
            // Retaining this parsing for now as it allows us to cross-check our impl.
            MySqlValueFormat::Text => value.as_str()?.parse().map_err(Into::into),
        }
    }
}

impl TryFrom<MySqlTime> for Time {
    type Error = BoxDynError;

    fn try_from(time: MySqlTime) -> Result<Self, Self::Error> {
        if !time.is_valid_time_of_day() {
            return Err(format!("Cannot convert `MySqlTime` value to jiff `Time`: {time}").into());
        }

        // `is_valid_time_of_day()` guarantees the ranges of these values
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        Ok(Time::new(
            time.hours() as i8,
            time.minutes() as i8,
            time.seconds() as i8,
            time.subsec_nanos() as i32,
        )?)
    }
}

impl From<MySqlTime> for SignedDuration {
    fn from(time: MySqlTime) -> Self {
        // `MySqlTime::subsec_nanos()` is at most 999_999_000
        #[allow(clippy::cast_possible_wrap)]
        let nanos = time.subsec_nanos() as i32;
        let nanos = if time.is_negative() { -nanos } else { nanos };

        SignedDuration::new(time.whole_seconds_signed(), nanos)
    }
}

impl TryFrom<SignedDuration> for MySqlTime {
    type Error = BoxDynError;

    fn try_from(value: SignedDuration) -> Result<Self, Self::Error> {
        let sign = if value.is_negative() {
            MySqlTimeSign::Negative
        } else {
            MySqlTimeSign::Positive
        };

        Ok(
            // `std::time::Duration` has a greater positive range than `SignedDuration`
            // which makes it a great intermediate if you ignore the sign.
            MySqlTime::try_from(value.unsigned_abs())?.with_sign(sign),
        )
    }
}

impl Type<MySql> for SignedDuration {
    fn type_info() -> MySqlTypeInfo {
        MySqlTime::type_info()
    }
}

impl<'r> Decode<'r, MySql> for SignedDuration {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(MySqlTime::decode(value)?.into())
    }
}

impl Type<MySql> for Date {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Date)
    }
}

impl Encode<'_, MySql> for Date {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.push(4);

        encode_date(self, buf)?;

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        5
    }
}

impl<'r> Decode<'r, MySql> for Date {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                // Row decoding should have left the length prefix.
                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                decode_date(&buf[1..])?.ok_or_else(|| UnexpectedNullError.into())
            }

            MySqlValueFormat::Text => value.as_str()?.parse().map_err(Into::into),
        }
    }
}

impl Type<MySql> for DateTime {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Datetime)
    }
}

impl Encode<'_, MySql> for DateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        let len = dt_encoded_len(self);
        buf.push(len);

        encode_date(&self.date(), buf)?;

        if len > 4 {
            encode_time(&self.time(), len > 7, buf);
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        dt_encoded_len(self) as usize + 1 // plus length byte
    }
}

impl<'r> Decode<'r, MySql> for DateTime {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                if buf.is_empty() {
                    return Err("empty buffer".into());
                }

                let len = buf[0];
                let date = decode_date(&buf[1..])?.ok_or(UnexpectedNullError)?;

                let dt = if len > 4 {
                    date.to_datetime(decode_time(len - 4, &buf[5..])?)
                } else {
                    date.to_datetime(Time::midnight())
                };

                Ok(dt)
            }

            MySqlValueFormat::Text => value.as_str()?.parse().map_err(Into::into),
        }
    }
}

fn encode_date(date: &Date, buf: &mut Vec<u8>) -> Result<(), BoxDynError> {
    // MySQL supports years 1000 - 9999
    let year = u16::try_from(date.year())
        .map_err(|_| format!("jiff Date out of range for Mysql: {date}"))?;

    buf.extend_from_slice(&year.to_le_bytes());

    // `Date` guarantees the ranges of these values
    #[allow(clippy::cast_sign_loss)]
    {
        buf.push(date.month() as u8);
        buf.push(date.day() as u8);
    }

    Ok(())
}

fn decode_date(mut buf: &[u8]) -> Result<Option<Date>, BoxDynError> {
    match buf.len() {
        // MySQL specifies that if there are no bytes, this is all zeros
        0 => Ok(None),
        4.. => {
            let year = buf.get_u16_le();
            let month = buf[0];
            let day = buf[1];

            let date = (|| {
                Date::new(
                    year.try_into().ok()?,
                    month.try_into().ok()?,
                    day.try_into().ok()?,
                )
                .ok()
            })()
            .ok_or_else(|| format!("server returned invalid date: {year}/{month}/{day}"))?;

            Ok(Some(date))
        }
        len => Err(format!("expected at least 4 bytes for date, got {len}").into()),
    }
}

fn encode_time(time: &Time, include_micros: bool, buf: &mut Vec<u8>) {
    // `Time` API guarantees the ranges of these values
    #[allow(clippy::cast_sign_loss)]
    {
        buf.push(time.hour() as u8);
        buf.push(time.minute() as u8);
        buf.push(time.second() as u8);
    }

    if include_micros {
        #[allow(clippy::cast_sign_loss)]
        buf.extend((time.subsec_nanosecond() as u32 / 1000).to_le_bytes());
    }
}

fn decode_time(len: u8, mut buf: &[u8]) -> Result<Time, BoxDynError> {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();

    let micros = if len > 3 {
        // microseconds : int<EOF>
        buf.get_uint_le(buf.len())
    } else {
        0
    };

    let nanos = i32::try_from(micros)
        .ok()
        .and_then(|micros| micros.checked_mul(1000))
        .ok_or_else(|| format!("server returned microseconds out of range: {micros}"))?;

    #[allow(clippy::cast_possible_wrap)]
    Time::new(hour as i8, minute as i8, seconds as i8, nanos).map_err(|_| {
        format!(
            "server returned invalid time: {hour:02}:{minute:02}:{seconds:02}; micros: {micros}"
        )
        .into()
    })
}

#[inline(always)]
fn dt_encoded_len(dt: &DateTime) -> u8 {
    // to save space the packet can be compressed:
    match (dt.hour(), dt.minute(), dt.second(), dt.subsec_nanosecond()) {
        // if hour, minutes, seconds and micro_seconds are all 0,
        // length is 4 and no other field is sent
        (0, 0, 0, 0) => 4,

        // if micro_seconds is 0, length is 7
        // and micro_seconds is not sent
        (_, _, _, 0) => 7,

        // otherwise length is 11
        (_, _, _, _) => 11,
    }
}

#[inline(always)]
fn time_encoded_len(time: &Time) -> u8 {
    if time.subsec_nanosecond() == 0 {
        // if micro_seconds is 0, length is 8 and micro_seconds is not sent
        8
    } else {
        // otherwise length is 12
        12
    }
}
