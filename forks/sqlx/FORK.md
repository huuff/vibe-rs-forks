# Fork of sqlx

- Upstream: https://github.com/launchbadge/sqlx
- Base: main at commit e1a6a2d (`sqlx` 0.9.1)

Everything outside the sections below is a plain copy of upstream. Keep
patches minimal and in upstream style; add a section here for each new one.

Consumers depend on the `sqlx` package:

```toml
sqlx = { git = "https://github.com/huuff/vibe-rs-forks", branch = "main", features = ["postgres", "runtime-tokio", "jiff"] }
```

## Differences from upstream

### `jiff` feature: date/time types from `jiff`

Upstream supports `chrono` and `time` but not `jiff`, and the orphan rule
stops a consumer from adding the impls. The optional `jiff` feature
(dependency `jiff` 1.0, `default-features = false, features = ["std"]`)
mirrors the `chrono` integration:

| jiff type               | Postgres                 | MySQL/MariaDB                 | SQLite                         |
|-------------------------|--------------------------|-------------------------------|--------------------------------|
| `Timestamp`             | TIMESTAMPTZ              | TIMESTAMP (assumes UTC conn.) | DATETIME (TEXT, INTEGER, REAL) |
| `civil::DateTime`       | TIMESTAMP                | DATETIME                      | DATETIME (TEXT, INTEGER, REAL) |
| `civil::Date`           | DATE                     | DATE                          | DATE (TEXT)                    |
| `civil::Time`           | TIME                     | TIME (time-of-day only)       | TIME (TEXT)                    |
| `SignedDuration`        | INTERVAL (encode only)   | TIME (decode only)            | –                              |
| `PgTimeTz<Time, Offset>`| TIMETZ                   | –                             | –                              |

Postgres also gets arrays of all of these and `PgRange<Date | DateTime |
Timestamp>`. `MySqlTime` converts to and from `SignedDuration` and into
`civil::Time`. `sqlx::types::jiff` re-exports `civil`, `tz`, `Timestamp`
and `SignedDuration`.

Behaviour choices, matching `chrono` where there was one:

- Postgres timestamps/times truncate sub-microsecond precision on encode,
  so `Timestamp::now()` binds; `SignedDuration` → `INTERVAL` errors on it,
  like `chrono::Duration`.
- Values outside jiff's range (`infinity`, year 10000+, `24:00:00`) fail
  to decode instead of panicking.
- Stored values are byte-identical to the
  [`jiff-sqlx`](https://crates.io/crates/jiff-sqlx) 1.0 wrapper crate, so a
  database can move between the two (checked by binding through one and
  casting to text). On SQLite this means jiff's `Display`:
  `2024-03-04T05:06:07.5Z` for `Timestamp`, `2024-03-04T05:06:07.5` for
  `civil::DateTime`, not chrono's `+00:00` / space-separated forms. SQLite
  compares `DATETIME` text lexicographically, so don't mix chrono- and
  jiff-written values in one column. Text decoding accepts both, plus
  every format chrono accepts.
- `Zoned` is not supported: no database column stores an IANA time zone.

`query!()` macros: `jiff` is a third date-time crate with the lowest
precedence (`time`, then `chrono`, then `jiff`); `sqlx.toml` accepts
`[macros.preferred-crates] date-time = "jiff"` (`DateTimeCrate::Jiff`).
With `Inferred` and more than one of the three enabled, the macros warn
about ambiguity as upstream does for `chrono` + `time`.

- Code: `sqlx-postgres/src/types/jiff/`, `sqlx-mysql/src/types/jiff.rs`,
  `sqlx-sqlite/src/types/jiff.rs`; jiff parts in
  `sqlx-postgres/src/types/{interval,range,time_tz}.rs`; macro tables in
  `sqlx-core/src/type_checking.rs` and each driver's `type_checking.rs`;
  `DateTimeCrate::Jiff` in `sqlx-core/src/config/macros.rs`; feature
  wiring in every `Cargo.toml`; ambiguity warning in `src/lib.rs`.
- Tests: unit tests in the files above (`cargo test -p sqlx-postgres
  --features jiff --lib`, same for `sqlx-mysql`, `sqlx-sqlite`); `mod jiff`
  in `tests/{postgres,mysql,sqlite}/types.rs`, run with e.g.
  `DATABASE_URL=postgres://... cargo test --features postgres,runtime-tokio,tls-none,macros,jiff --test postgres-types`.
- User docs: the type tables in each driver's `src/types/mod.rs`.

### Fix: quoted bounds in Postgres range text format

`PgRange` text decoding read `["a","b")` as an escaped `""` and put a
stray `"` into the upper bound, so any range whose bounds Postgres quotes
(e.g. `tstzrange`) failed to decode in text format (unprepared queries),
for `chrono` and `time` too. One-line fix in
`sqlx-postgres/src/types/range.rs`; covered by
`jiff_timestamp_range` in `tests/postgres/types.rs`.

### Fix: MySQL `TIME '-00:...'` lost its sign in text format

`MySqlTime` text parsing took the sign from the parsed hours, so
`-00:00:01.5` decoded as positive (also for `chrono::TimeDelta`). The sign
now comes from the text, and an all-zero value stays positive since
`MySqlTime` has no negative zero. Fix and unit test in
`sqlx-mysql/src/types/mysql_time.rs`; also covered by
`jiff_signed_duration` in `tests/mysql/types.rs`.
