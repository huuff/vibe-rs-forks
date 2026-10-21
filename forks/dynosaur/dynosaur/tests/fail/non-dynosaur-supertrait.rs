use dynosaur::dynosaur;

// `Bar` has methods but no `#[dynosaur]` attribute, so `Foo` cannot learn
// them: the generated `__dynosaur_Bar!` call has nothing to resolve to.
trait Bar {
    fn bar(&self) -> i32;
}

#[dynosaur(DynFoo = dyn(box) Foo)]
trait Foo: Bar {
    async fn foo(&self) -> i32;
}

fn main() {}
