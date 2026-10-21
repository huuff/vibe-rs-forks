use dynosaur::dynosaur;

// `_` generates no `Dyn*` type, so there is nothing for `bridge` to apply to.
#[dynosaur(_, bridge(none))]
trait Foo {
    fn foo(&self) -> i32;
}

fn main() {}
