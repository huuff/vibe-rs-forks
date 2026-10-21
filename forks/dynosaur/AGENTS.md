# Fork of dynosaur

- Upstream: https://github.com/spastorino/dynosaur
- Base: v0.3.1 (commit 0663970)

Everything outside the sections below is a plain copy of upstream. Keep
patches minimal and in upstream style; add a section here for each new one.

## Differences from upstream

### Supertraits that are themselves `#[dynosaur]` traits

Upstream rejects `trait Foo: Bar` when `Bar` has methods (issue #79). The
fork makes `DynFoo` implement `Bar` too. Each `#[dynosaur]` trait emits a
hidden `macro_rules! __dynosaur_<Trait>` carrying its definition; a
subtrait's expansion calls it, and the `__resolve_supertraits!` proc macro
finishes the expansion with the supertrait's items merged in.

- Code: `dynosaur_derive/src/supertraits.rs`, `__resolve_supertraits` in
  `dynosaur_derive/src/lib.rs`.
- Tests: `dynosaur/tests/pass/supertraits*.rs`,
  `dynosaur/tests/fail/*supertrait*.rs`.
- User docs: "Supertraits" section of the `dynosaur` macro doc comment.
- Limits: same crate only (`__dynosaur_<Trait>` is at most `pub(crate)`),
  no generic supertraits, no `const` items (they make the trait not dyn
  compatible).

### Carried receivers get the subtrait's hygiene

Definitions that arrive through the callback `macro_rules!` carry its
syntax context. `self` is hygienic, so an attribute macro that synthesizes
`self` on the generated items (`mockall::automock`, which dynosaur copies
onto the erased trait) could not see the merged methods' receivers:
"cannot find value `self` in this scope". `respan_receivers` moves them to
the subtrait's span before expansion.

- Code: `respan_receivers` in `dynosaur_derive/src/supertraits.rs`.
- Test: `dynosaur/tests/supertraits-automock.rs`, a plain cargo test (the
  mock expansion is too toolchain-dependent for a golden file). It needs
  the `mockall` dev-dependency in `dynosaur/tests/Cargo.toml`.

### `#[dynosaur(_)]`

Registers a trait as a possible supertrait without generating a `Dyn*` type
(which would otherwise sit unused and trip `dead_code` on its erased
trait). Expands to the trait plus its callback macro only. `_` takes no
options.

- Code: `Attrs::Discard` in `dynosaur_derive/src/lib.rs`.
- Tests: `dynosaur/tests/pass/supertraits-discard.rs`,
  `dynosaur/tests/fail/discard-with-bridge.rs`.

## Testing

```sh
cargo test --workspace --locked          # incl. ui tests; works on stable via RUSTC_BOOTSTRAP
BLESS=1 cargo test -p dynosaur_ui_tests  # regenerate tests/pass/*.stdout and tests/fail/*.stderr
```

UI tests pin the toolchain in `dynosaur/tests/ui-toolchain.txt`; golden
files drift across rustc versions, so re-bless only on that toolchain.
