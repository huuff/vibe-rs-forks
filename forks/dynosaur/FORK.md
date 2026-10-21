# Fork of dynosaur

- Upstream: https://github.com/spastorino/dynosaur
- Base: v0.3.1 (commit 0663970)

## Changes

- Supertraits that are themselves `#[dynosaur]` traits are supported:
  `trait Foo: Bar` generates `impl Bar for DynFoo` (upstream issue #79).
  Mechanism: each `#[dynosaur]` trait emits a hidden `macro_rules!
  __dynosaur_<Trait>` carrying its definition; a subtrait's expansion calls
  it and the `__resolve_supertraits!` proc macro finishes the job. Code in
  `dynosaur_derive/src/supertraits.rs`; tests `tests/pass/supertraits*.rs`,
  `tests/fail/*supertrait*.rs`; docs in the `dynosaur` macro doc comment.
- Receivers of carried supertrait methods are re-spanned to the subtrait's
  hygiene context, so attribute macros that synthesize `self` (e.g.
  `mockall::automock`, which dynosaur copies onto the erased trait) can see
  them. Regression test: `tests/supertraits-automock.rs` (plain cargo test,
  needs the `mockall` dev-dependency).

## Testing

```sh
cargo test --workspace --locked          # incl. ui tests; works on stable via RUSTC_BOOTSTRAP
BLESS=1 cargo test -p dynosaur_ui_tests  # regenerate tests/pass/*.stdout
```
