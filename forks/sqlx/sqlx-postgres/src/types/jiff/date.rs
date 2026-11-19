use std::mem;

use jiff::civil::Date;
use jiff::SignedDuration;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

impl Type<Postgres> for Date {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::DATE
    }
}

impl PgHasArrayType for Date {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::DATE_ARRAY
    }
}

impl Encode<'_, Postgres> for Date {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // DATE is encoded as the days since epoch
        let days: i32 = self.since(postgres_epoch_date())?.get_days();
        Encode::<Postgres>::encode(days, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i32>()
    }
}

impl<'r> Decode<'r, Postgres> for Date {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::<Postgres>::decode(value)?;

                // `infinity`, `-infinity` and years beyond jiff's range don't fit in `Date`
                postgres_epoch_date()
                    .checked_add(SignedDuration::from_hours(i64::from(days) * 24))
                    .map_err(|_| {
                        format!(
                            "Postgres DATE out of range for jiff Date: {days} days from 2000-01-01"
                        )
                    })?
            }

            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

#[inline]
fn postgres_epoch_date() -> Date {
    Date::constant(2000, 1, 1)
}
