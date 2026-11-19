use crate::faker::jiff::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use jiff::tz::TimeZone;
use jiff::SignedDuration;
use rand::RngExt;

const MINUTES_MAX_BOUND: i64 = 1_000_000;

fn format_utc(fmt: &str, timestamp: jiff::Timestamp) -> String {
    jiff::fmt::strtime::format(fmt, &timestamp.to_zoned_datetime(TimeZone::UTC))
        .expect("failed to format timestamp")
}

impl<L: Data> Dummy<Time<L>> for jiff::civil::Time {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Time<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Time<L>, rng: &mut R) -> Self {
        let time: jiff::civil::Time = Faker.fake_with_rng(rng);
        jiff::fmt::strtime::format(L::JIFF_DEFAULT_TIME_FORMAT, time)
            .expect("failed to format time")
    }
}

impl<L: Data> Dummy<Date<L>> for jiff::civil::Date {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<Date<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Date<L>, rng: &mut R) -> Self {
        let date: jiff::civil::Date = Faker.fake_with_rng(rng);
        jiff::fmt::strtime::format(L::JIFF_DEFAULT_DATE_FORMAT, date)
            .expect("failed to format date")
    }
}

impl<L: Data> Dummy<DateTime<L>> for jiff::civil::DateTime {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for jiff::Timestamp {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for jiff::ZonedDateTime {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTime<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &DateTime<L>, rng: &mut R) -> Self {
        format_utc(L::JIFF_DEFAULT_DATETIME_FORMAT, Faker.fake_with_rng(rng))
    }
}

impl<L: Data> Dummy<Duration<L>> for SignedDuration {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Duration<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for jiff::Timestamp {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        c.1 - SignedDuration::from_mins(mins)
    }
}

impl<L: Data> Dummy<DateTimeBefore<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBefore<L>, rng: &mut R) -> Self {
        format_utc(L::JIFF_DEFAULT_DATETIME_FORMAT, c.fake_with_rng(rng))
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for jiff::Timestamp {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        let mins: i64 = (1..MINUTES_MAX_BOUND).fake_with_rng(rng);
        c.1 + SignedDuration::from_mins(mins)
    }
}

impl<L: Data> Dummy<DateTimeAfter<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeAfter<L>, rng: &mut R) -> Self {
        format_utc(L::JIFF_DEFAULT_DATETIME_FORMAT, c.fake_with_rng(rng))
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for jiff::Timestamp {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        let max_minutes = c.2.duration_since(c.1).as_mins();

        let from_start: i64 = (0..max_minutes).fake_with_rng(rng);
        c.1 + SignedDuration::from_mins(from_start)
    }
}

impl<L: Data> Dummy<DateTimeBetween<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &DateTimeBetween<L>, rng: &mut R) -> Self {
        format_utc(L::JIFF_DEFAULT_DATETIME_FORMAT, c.fake_with_rng(rng))
    }
}
