use crate::value::ValueRef;
use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    type_info::DataType,
    types::Type,
    Sqlite, SqliteArgumentsBuffer, SqliteTypeInfo, SqliteValueRef,
};
use jiff::civil::{Date, DateTime, Time};
use jiff::fmt::temporal::{Pieces, PiecesOffset};
use jiff::tz::Offset;
use jiff::Timestamp;

impl Type<Sqlite> for Timestamp {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Datetime)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <DateTime as Type<Sqlite>>::compatible(ty)
    }
}

impl Type<Sqlite> for DateTime {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Datetime)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(
            ty.0,
            DataType::Datetime
                | DataType::Text
                | DataType::Integer
                | DataType::Int4
                | DataType::Float
        )
    }
}

impl Type<Sqlite> for Date {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Date)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Date | DataType::Text)
    }
}

impl Type<Sqlite> for Time {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Time)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Time | DataType::Text)
    }
}

/// Encodes as `YYYY-MM-DDTHH:MM:SS[.f]Z`, the same text as `jiff-sqlx`.
impl Encode<'_, Sqlite> for Timestamp {
    fn encode_by_ref(&self, buf: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        Encode::<Sqlite>::encode(self.to_string(), buf)
    }
}

/// Encodes as `YYYY-MM-DDTHH:MM:SS[.f]`, the same text as `jiff-sqlx`.
impl Encode<'_, Sqlite> for DateTime {
    fn encode_by_ref(&self, buf: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        Encode::<Sqlite>::encode(self.to_string(), buf)
    }
}

impl Encode<'_, Sqlite> for Date {
    fn encode_by_ref(&self, buf: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        Encode::<Sqlite>::encode(self.to_string(), buf)
    }
}

impl Encode<'_, Sqlite> for Time {
    fn encode_by_ref(&self, buf: &mut SqliteArgumentsBuffer) -> Result<IsNull, BoxDynError> {
        Encode::<Sqlite>::encode(self.to_string(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Timestamp {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let (dt, offset) = decode_datetime(value)?;
        Ok(offset.to_timestamp(dt)?)
    }
}

impl<'r> Decode<'r, Sqlite> for DateTime {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(decode_datetime(value)?.0)
    }
}

/// Decodes the civil date-time as written, and its offset (UTC if none was written).
fn decode_datetime(value: SqliteValueRef<'_>) -> Result<(DateTime, Offset), BoxDynError> {
    let dt = match value.type_info().0 {
        DataType::Text => decode_datetime_from_text(value.text_borrowed()?),
        DataType::Int4 | DataType::Integer => {
            Timestamp::from_second(value.int64()?).ok().map(from_utc)
        }
        DataType::Float => decode_datetime_from_float(value.double()?).map(from_utc),

        _ => None,
    };

    if let Some(dt) = dt {
        Ok(dt)
    } else {
        Err(format!("invalid datetime: {}", value.text_borrowed()?).into())
    }
}

fn from_utc(ts: Timestamp) -> (DateTime, Offset) {
    (Offset::UTC.to_datetime(ts), Offset::UTC)
}

/// Accepts RFC 3339 and the formats in SQLite's docs: `T` or space separator, optional seconds
/// and fraction, and an optional `Z` or `±HH:MM` offset.
fn decode_datetime_from_text(value: &str) -> Option<(DateTime, Offset)> {
    let pieces = Pieces::parse(value).ok()?;
    let dt = pieces.date().to_datetime(pieces.time()?);
    let offset = match pieces.offset() {
        None | Some(PiecesOffset::Zulu) => Offset::UTC,
        Some(PiecesOffset::Numeric(offset)) => offset.offset(),
        Some(_) => return None,
    };

    Some((dt, offset))
}

/// Julian day number, as returned by `julianday()`.
fn decode_datetime_from_float(value: f64) -> Option<Timestamp> {
    let epoch_in_julian_days = 2_440_587.5;
    let seconds_in_day = 86400.0;
    let timestamp = (value - epoch_in_julian_days) * seconds_in_day;

    if !timestamp.is_finite() {
        return None;
    }

    // We don't really have a choice but to do lossy casts for this conversion
    // We checked above if the value is infinite or NaN which could otherwise cause problems
    #[allow(clippy::cast_possible_truncation)]
    {
        // Round down so the sub-second part is non-negative, as for `chrono`.
        let seconds = timestamp.floor();
        let nanos = ((timestamp - seconds) * 1E9) as i32;

        Timestamp::new(seconds as i64, nanos).ok()
    }
}

impl<'r> Decode<'r, Sqlite> for Date {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Date::strptime("%F", value.text_borrowed()?)?)
    }
}

impl<'r> Decode<'r, Sqlite> for Time {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let value = value.text_borrowed()?;

        // Same formats as `chrono`: `HH:MM[:SS[.f]]`, optionally followed by `Z` or `±HH:MM`,
        // which is ignored.
        let time = value
            .strip_suffix('Z')
            .or_else(|| {
                let (time, offset) = value.split_at(value.len().checked_sub(6)?);
                (offset.starts_with(['+', '-']) && offset.as_bytes()[3] == b':').then_some(time)
            })
            .unwrap_or(value);

        time.parse()
            .map_err(|_| format!("invalid time: {value}").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::{date, time};

    #[test]
    fn decodes_text_datetime_formats() {
        let dt = date(2020, 1, 2).at(3, 4, 5, 600_000_000);
        let dt_min = date(2020, 1, 2).at(3, 4, 0, 0);
        let plus2 = Offset::constant(2);

        for (text, expected) in [
            ("2020-01-02 03:04:05.6", (dt, Offset::UTC)),
            ("2020-01-02 03:04", (dt_min, Offset::UTC)),
            ("2020-01-02 03:04Z", (dt_min, Offset::UTC)),
            ("2020-01-02 03:04+02:00", (dt_min, plus2)),
            ("2020-01-02 03:04:05.6Z", (dt, Offset::UTC)),
            ("2020-01-02 03:04:05.6+02:00", (dt, plus2)),
            ("2020-01-02T03:04", (dt_min, Offset::UTC)),
            ("2020-01-02T03:04:05.6+02:00", (dt, plus2)),
        ] {
            assert_eq!(decode_datetime_from_text(text), Some(expected), "{text}");
        }

        assert_eq!(decode_datetime_from_text("2020-01-02"), None);
    }

    #[test]
    fn decodes_pre_epoch_float_datetime() {
        // 1969-12-31 23:59:59.500 UTC
        let decoded = decode_datetime_from_float(2_440_587.5 - 0.5 / 86_400.0).unwrap();
        let expected = Timestamp::new(-1, 500_000_000).unwrap();
        assert!(decoded.duration_since(expected).abs().as_micros() < 1_000);
    }

    // Must match `jiff-sqlx` byte for byte: SQLite compares these as text.
    #[test]
    fn encodes_like_jiff_sqlx() {
        let dt = date(2020, 1, 2).at(3, 4, 5, 600_000_000);
        assert_eq!(dt.to_string(), "2020-01-02T03:04:05.6");
        assert_eq!(dt.date().to_string(), "2020-01-02");
        assert_eq!(time(3, 4, 5, 0).to_string(), "03:04:05");
        assert_eq!(
            Timestamp::new(0, 123_456_000).unwrap().to_string(),
            "1970-01-01T00:00:00.123456Z"
        );
    }
}
