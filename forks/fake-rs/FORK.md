# Fork of fake-rs

- Upstream: https://github.com/cksac/fake-rs
- Base: master at commit 78d40b3 (`fake` 5.1.0, `dummy` 0.12.0)

Everything outside the sections below is a plain copy of upstream. Keep
patches minimal and in upstream style; add a section here for each new one.

Consumers depend on the `fake` package:

```toml
fake = { git = "https://github.com/huuff/vibe-rs-forks", branch = "main", features = ["derive", "nonempty", "jiff"] }
```

## Differences from upstream

### `nonempty` feature: `Dummy` for `nonempty::NonEmpty<T>`

Upstream cannot fake `NonEmpty<T>` from the `nonempty` crate, and the orphan
rule stops a consumer from adding the impl. The optional `nonempty` feature
(dependency `nonempty` 0.12) adds two impls, mirroring `Vec<T>`:

- `Dummy<Faker>`: a faked head plus a `Vec<T>` tail, so 1 to 10 elements.
  `#[derive(Dummy)]` picks this up for `NonEmpty` fields.
- `Dummy<(E, L)>`: `E` fakes every element, `L` gives the total length, head
  included. A length of 0 gives one element instead of panicking, so a range
  such as `0..3` is safe.

The consumer must resolve to the same `nonempty` 0.12 as this fork; with
another minor version the impls are for a different type.

- Code: `fake/src/impls/nonempty/mod.rs`, registered in
  `fake/src/impls/mod.rs`; dependency in `fake/Cargo.toml`.
- Tests: `fake/tests/nonempty.rs` (`cargo test -p fake --features nonempty`).
- User docs: the feature list in `README.md` and `fake/README.md`.

### `jiff` feature: `Dummy` for `jiff` date/time types

Upstream supports `chrono` and `time` but not `jiff` (upstream issue
cksac/fake-rs#271), and the orphan rule stops a consumer from adding the
impls. The optional `jiff` feature (dependency `jiff` 1,
`default-features = false, features = ["std"]`) mirrors the `chrono`
integration:

- `Dummy<Faker>` for `Timestamp`, `ZonedDateTime`, `SignedDuration`,
  `civil::{Date, Time, DateTime, Weekday}` and `tz::{Offset, TimeZone}`.
  Like chrono: `Timestamp` comes from a random `i64` of nanoseconds (years
  1677–2262), `Date` has years 1–2999, `Time` has whole seconds, `Offset`
  is -12:00 to +14:00 in half hours, and `TimeZone`/`ZonedDateTime` use a
  fixed offset (no IANA zones, so no tzdb is needed). Unlike chrono, `Date`
  can produce the last day of the year.
- `fake::jiff::Precision<N>` (N = 0..=9 fractional digits) for `Time`,
  `DateTime`, `Timestamp` and `ZonedDateTime`, e.g.
  `#[dummy(faker = "Precision::<6>")]` for values that must round-trip
  through Postgres.
- `fake::faker::jiff::<locale>::{Time, Date, DateTime, Duration,
  DateTimeBefore, DateTimeAfter, DateTimeBetween}`, like
  `faker::chrono`; the bounds are `Timestamp`s, and every faker also
  produces a `String`.
- Strings use jiff's strftime with new `locales::Data` constants
  `JIFF_DEFAULT_{TIME,DATE,DATETIME}_FORMAT`. Time and date default to the
  locale's chrono formats. jiff has no `%+`, so the date-time default is
  `%Y-%m-%dT%H:%M:%S%.f%:z` (formatted in UTC), and the six locales that
  override the chrono date-time format (`bn_bd`, `fa_ir`, `it_it`,
  `pt_pt`, `tr_tr`, `vi`) set the jiff one to it.

jiff 1.0 renamed `Zoned` to `ZonedDateTime`. The consumer must resolve to
the same `jiff` 1.x as this fork (jiff 0.2 types are different types).

- Code: `fake/src/impls/jiff/mod.rs`, `fake/src/faker/impls/jiff.rs`,
  `def_fakers!` block in `fake/src/faker/mod.rs`, constants in
  `fake/src/locales/mod.rs` and the six locales, re-export and feature doc
  in `fake/src/lib.rs`, registration in both `impls/mod.rs`; dependency in
  `fake/Cargo.toml`.
- Tests: `fake/tests/jiff.rs` (`cargo test -p fake --features jiff,derive`).
- User docs: the feature list in `README.md` and `fake/README.md`.
