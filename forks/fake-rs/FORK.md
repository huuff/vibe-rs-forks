# Fork of fake-rs

- Upstream: https://github.com/cksac/fake-rs
- Base: master at commit 78d40b3 (`fake` 5.1.0, `dummy` 0.12.0)

Everything outside the sections below is a plain copy of upstream. Keep
patches minimal and in upstream style; add a section here for each new one.

Consumers depend on the `fake` package:

```toml
fake = { git = "https://github.com/haf/vibe-rs-forks", branch = "main", features = ["derive", "nonempty"] }
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
