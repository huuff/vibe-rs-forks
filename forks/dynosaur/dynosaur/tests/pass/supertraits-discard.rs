use dynosaur::dynosaur;

// `#[dynosaur(_)]` registers a trait as a possible supertrait without
// generating a `Dyn*` type for it. It composes transitively.
#[dynosaur(_)]
trait Base {
    fn base(&self) -> i32;
}

#[dynosaur(_)]
trait Bar: Base {
    fn bar(&self) -> i32;
}

#[dynosaur(DynFoo = dyn(box) Foo)]
trait Foo: Bar {
    async fn foo(&self) -> i32;
}

struct S;

impl Base for S {
    fn base(&self) -> i32 {
        0
    }
}

impl Bar for S {
    fn bar(&self) -> i32 {
        1
    }
}

impl Foo for S {
    async fn foo(&self) -> i32 {
        self.bar() + 1
    }
}

async fn use_dyn(f: &DynFoo<'_>) -> (i32, i32, i32) {
    (f.base(), f.bar(), f.foo().await)
}

fn main() {
    let s = S;
    let _ = use_dyn(DynFoo::from_ref(&s));
}
