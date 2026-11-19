use std::mem;

use jiff::civil::{date, DateTime};
use jiff::{SignedDuration, Timestamp};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

impl Type<Postgres> for DateTime {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TIMESTAMP
    }
}

impl Type<Postgres> for Timestamp {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TIMESTAMPTZ
    }
}

impl PgHasArrayType for DateTime {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIMESTAMP_ARRAY
    }
}

impl PgHasArrayType for Timestamp {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIMESTAMPTZ_ARRAY
    }
}

impl Encode<'_, Postgres> for DateTime {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIMESTAMP is encoded as the microseconds since the epoch
        let micros = micros_from_duration(self.duration_since(postgres_epoch_datetime()))?;
        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Postgres> for DateTime {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let us: i64 = Decode::<Postgres>::decode(value)?;

                // `infinity`, `-infinity` and years beyond jiff's range don't fit in `DateTime`
                postgres_epoch_datetime()
                    .checked_add(SignedDuration::from_micros(us))
                    .map_err(|_| {
                        format!("Postgres TIMESTAMP out of range for jiff DateTime: {us} microseconds from 2000-01-01")
                    })?
            }

            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Encode<'_, Postgres> for Timestamp {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIMESTAMPTZ is encoded as the microseconds since the epoch, in UTC
        let micros = micros_from_duration(self.duration_since(postgres_epoch_timestamp()))?;
        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Postgres> for Timestamp {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMPTZ is encoded as the microseconds since the epoch, in UTC
                let us: i64 = Decode::<Postgres>::decode(value)?;

                // `infinity`, `-infinity` and years beyond jiff's range don't fit in `Timestamp`
                postgres_epoch_timestamp()
                    .checked_add(SignedDuration::from_micros(us))
                    .map_err(|_| {
                        format!("Postgres TIMESTAMPTZ out of range for jiff Timestamp: {us} microseconds from 2000-01-01")
                    })?
            }

            // Postgres sends the offset as `+HH`, `+HH:MM` or `+HH:MM:SS`
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

/// Truncates sub-microsecond precision, which Postgres does not store.
fn micros_from_duration(d: SignedDuration) -> Result<i64, BoxDynError> {
    Ok(d.as_micros().try_into()?)
}

#[inline]
fn postgres_epoch_datetime() -> DateTime {
    date(2000, 1, 1).at(0, 0, 0, 0)
}

#[inline]
fn postgres_epoch_timestamp() -> Timestamp {
    Timestamp::constant(946_684_800, 0)
}
