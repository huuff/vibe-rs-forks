use dynosaur::dynosaur;

#[dynosaur(DynBar = dyn(box) Bar)]
trait Bar {
    type Out;
    fn bar(&self) -> i32;
    async fn baz(&self) -> Self::Out;
}

#[dynosaur(DynFoo = dyn(box) Foo)]
trait Foo: Bar + Send {
    async fn foo(&self) -> i32;
}

struct S;

impl Bar for S {
    type Out = u8;
    fn bar(&self) -> i32 {
        1
    }
    async fn baz(&self) -> u8 {
        2
    }
}

impl Foo for S {
    async fn foo(&self) -> i32 {
        self.bar() + 1
    }
}

async fn use_dyn(f: &DynFoo<'_, u8>) -> (i32, u8, i32) {
    (f.bar(), f.baz().await, f.foo().await)
}

fn main() {
    let s = S;
    let _ = use_dyn(DynFoo::from_ref(&s));
}
