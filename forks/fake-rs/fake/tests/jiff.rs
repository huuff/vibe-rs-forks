#![cfg(feature = "jiff")]

use fake::faker::jiff::en::{Date, DateTime, DateTimeAfter, DateTimeBefore, DateTimeBetween, Time};
use fake::jiff::Precision;
use fake::{Dummy, Fake, Faker};
use jiff::civil;
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn faker_covers_every_type() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..1000 {
        let _: SignedDuration = Faker.fake_with_rng(&mut rng);
        let _: Timestamp = Faker.fake_with_rng(&mut rng);
        let _: ZonedDateTime = Faker.fake_with_rng(&mut rng);
        let _: TimeZone = Faker.fake_with_rng(&mut rng);
        let _: civil::Weekday = Faker.fake_with_rng(&mut rng);
        let _: civil::DateTime = Faker.fake_with_rng(&mut rng);

        let date: civil::Date = Faker.fake_with_rng(&mut rng);
        assert!((1..3000).contains(&date.year()), "{date}");

        let time: civil::Time = Faker.fake_with_rng(&mut rng);
        assert_eq!(time.subsec_nanosecond(), 0);

        let offset: Offset = Faker.fake_with_rng(&mut rng);
        assert_eq!(offset.seconds() % 1800, 0);
        assert!((-12 * 3600..=14 * 3600).contains(&offset.seconds()));
    }
}

#[test]
fn date_reaches_the_last_day_of_the_month() {
    let mut rng = StdRng::seed_from_u64(42);
    let last_days = (0..10_000)
        .map(|_| Faker.fake_with_rng::<civil::Date, _>(&mut rng))
        .filter(|d| d.day() == d.days_in_month())
        .count();
    assert!(last_days > 0);
}

#[test]
fn precision_truncates_subseconds() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut nonzero = false;
    for _ in 0..1000 {
        let ts: Timestamp = Precision::<6>.fake_with_rng(&mut rng);
        assert_eq!(ts.subsec_nanosecond() % 1000, 0);

        let zdt: ZonedDateTime = Precision::<3>.fake_with_rng(&mut rng);
        assert_eq!(zdt.subsec_nanosecond() % 1_000_000, 0);

        let dt: civil::DateTime = Precision::<0>.fake_with_rng(&mut rng);
        assert_eq!(dt.subsec_nanosecond(), 0);

        let time: civil::Time = Precision::<9>.fake_with_rng(&mut rng);
        nonzero |= time.subsec_nanosecond() != 0;
    }
    assert!(nonzero);
}

#[test]
fn before_after_between() {
    let start: Timestamp = "2024-01-01T00:00:00Z".parse().unwrap();
    let end: Timestamp = "2024-02-01T00:00:00Z".parse().unwrap();
    for _ in 0..100 {
        let ts: Timestamp = DateTimeBefore(start).fake();
        assert!(ts < start);
        let ts: Timestamp = DateTimeAfter(start).fake();
        assert!(ts > start);
        let ts: Timestamp = DateTimeBetween(start, end).fake();
        assert!(start <= ts && ts < end);
    }
}

#[test]
fn strings_parse_back() {
    for _ in 0..100 {
        let s: String = Date().fake();
        s.parse::<civil::Date>().unwrap();
        let s: String = Time().fake();
        s.parse::<civil::Time>().unwrap();
        let s: String = DateTime().fake();
        s.parse::<Timestamp>().unwrap();
    }
}

#[test]
fn locale_formats() {
    use fake::faker::jiff::pt_pt;
    let s: String = pt_pt::DateTime().fake();
    civil::DateTime::strptime("%d-%m-%Y %H:%M:%S", &s).unwrap();
    let s: String = pt_pt::Date().fake();
    civil::Date::strptime("%d-%m-%Y", &s).unwrap();
}

#[derive(Dummy)]
#[allow(dead_code)]
struct Event {
    at: Timestamp,
    zoned: ZonedDateTime,
    day: civil::Date,
    #[dummy(faker = "Precision::<6>")]
    local: civil::DateTime,
    length: SignedDuration,
}

#[test]
fn derive() {
    let _: Event = Faker.fake();
}

#[test]
fn every_locale_formats() {
    use fake::faker::jiff::raw;
    use fake::locales::*;
    macro_rules! check {
        ($($l:expr),*) => {$(
            let _: String = raw::Time($l).fake();
            let _: String = raw::Date($l).fake();
            let _: String = raw::DateTime($l).fake();
        )*};
    }
    check!(
        EN, FR_FR, ZH_TW, ZH_CN, AR_SA, JA_JP, PT_BR, PT_PT, DE_DE, IT_IT, CY_GB, NL_NL, TR_TR,
        FA_IR, BN_BD
    );
}
