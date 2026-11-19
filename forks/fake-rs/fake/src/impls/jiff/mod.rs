use crate::{Dummy, Fake, Faker};
use jiff::civil::{Date, DateTime, Time, Weekday};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use rand::RngExt;

const YEAR_MAG: i16 = 3_000i16;

impl Dummy<Faker> for SignedDuration {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        SignedDuration::from_nanos(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for Offset {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        // -12:00 to +14:00 in half hours, like chrono's FixedOffset
        let halves: i32 = (-24..=28).fake_with_rng(rng);
        Offset::from_seconds(halves * 30 * 60).expect("failed to create Offset")
    }
}

impl Dummy<Faker> for TimeZone {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        TimeZone::fixed(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for Timestamp {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let nanos: i64 = Faker.fake_with_rng(rng);
        Timestamp::from_nanosecond(nanos.into()).expect("failed to create Timestamp")
    }
}

impl Dummy<Faker> for ZonedDateTime {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let timestamp: Timestamp = Faker.fake_with_rng(rng);
        ZonedDateTime::new(timestamp, Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for Weekday {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Weekday::from_monday_one_offset((1..=7).fake_with_rng(rng))
            .expect("failed to create Weekday")
    }
}

impl Dummy<Faker> for Date {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let year: i16 = (1..YEAR_MAG).fake_with_rng(rng);
        let month: i8 = (1..=12).fake_with_rng(rng);
        let first = Date::new(year, month, 1).expect("failed to create Date");
        let day: i8 = (1..=first.days_in_month()).fake_with_rng(rng);
        Date::new(year, month, day).expect("failed to create Date")
    }
}

impl Dummy<Faker> for Time {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        Time::new(hour, min, sec, 0).expect("failed to create Time")
    }
}

impl Dummy<Faker> for DateTime {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let date: Date = Faker.fake_with_rng(rng);
        date.to_datetime(Faker.fake_with_rng(rng))
    }
}

pub struct Precision<const N: usize>;

trait AllowedPrecision {
    const SCALE: i64;

    fn to_scale(nanos: i64) -> i64 {
        if nanos != 0 {
            nanos / Self::SCALE * Self::SCALE
        } else {
            nanos
        }
    }
}
macro_rules! allow_precision {
    ($($precision:expr),*) => {
        $(impl AllowedPrecision for Precision<$precision> {
            const SCALE: i64 = 10i64.pow(9 - $precision);
        })*
    };
}
allow_precision!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

impl<const N: usize> Dummy<Precision<N>> for Time
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let hour = (0..24).fake_with_rng(rng);
        let min = (0..60).fake_with_rng(rng);
        let sec = (0..60).fake_with_rng(rng);
        let nanos: i64 = (0..1_000_000_000).fake_with_rng(rng);
        let nanos = Precision::<N>::to_scale(nanos) as i32;
        Time::new(hour, min, sec, nanos).expect("failed to create Time")
    }
}

impl<const N: usize> Dummy<Precision<N>> for DateTime
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(precision: &Precision<N>, rng: &mut R) -> Self {
        let date: Date = Faker.fake_with_rng(rng);
        date.to_datetime(Precision::<N>::fake_with_rng(precision, rng))
    }
}

impl<const N: usize> Dummy<Precision<N>> for Timestamp
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Precision<N>, rng: &mut R) -> Self {
        let nanos: i64 = Faker.fake_with_rng(rng);
        Timestamp::from_nanosecond(Precision::<N>::to_scale(nanos).into())
            .expect("failed to create Timestamp")
    }
}

impl<const N: usize> Dummy<Precision<N>> for ZonedDateTime
where
    Precision<N>: AllowedPrecision,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(precision: &Precision<N>, rng: &mut R) -> Self {
        let timestamp: Timestamp = Precision::<N>::fake_with_rng(precision, rng);
        ZonedDateTime::new(timestamp, Faker.fake_with_rng(rng))
    }
}
