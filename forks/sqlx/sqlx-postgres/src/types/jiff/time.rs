use std::mem;

use jiff::civil::Time;
use jiff::SignedDuration;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

impl Type<Postgres> for Time {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TIME
    }
}

impl PgHasArrayType for Time {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TIME_ARRAY
    }
}

impl Encode<'_, Postgres> for Time {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // TIME is encoded as the microseconds since midnight
        let micros: i64 = self
            .duration_since(Time::midnight())
            .as_micros()
            .try_into()
            .map_err(|_| format!("Time out of range for PostgreSQL: {self}"))?;

        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl<'r> Decode<'r, Postgres> for Time {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us: i64 = Decode::<Postgres>::decode(value)?;
                time_from_micros(us)?
            }

            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

/// Postgres allows `24:00:00`, which would otherwise wrap around to midnight.
pub(crate) fn time_from_micros(us: i64) -> Result<Time, BoxDynError> {
    Time::midnight()
        .checked_add(SignedDuration::from_micros(us))
        .map_err(|_| {
            format!("Postgres TIME out of range for jiff Time: {us} microseconds from midnight")
                .into()
        })
}
