// Supertraits across modules (by path and by named import), transitively
// (`Foo: Bar: Qux`), and with arguments that must stay hygienic through the
// callback macro. A transitive supertrait is referred to by the path written
// in the intermediate trait (`Qux` in `Bar: Qux`), so it must be in scope
// where the subtrait is defined; rustc suggests the `use` when it is not.
mod base {
    #[dynosaur::dynosaur(pub DynQux = dyn(box) Qux)]
    pub trait Qux: Send {
        fn qux(&self, x: i32) -> i32;
    }

    #[dynosaur::dynosaur(pub DynBar = dyn(box) Bar)]
    pub trait Bar: Qux {
        async fn bar(&self, x: i32, _: u8) -> i32;
    }
}

mod by_path {
    use crate::base::Qux;

    #[dynosaur::dynosaur(pub DynFoo = dyn(box) Foo)]
    pub trait Foo: crate::base::Bar {
        async fn foo(&self) -> i32;
    }
}

mod by_import {
    use crate::base::{Bar, Qux, __dynosaur_Bar};

    #[dynosaur::dynosaur(pub DynFoo = dyn(box) Foo)]
    pub trait Foo: Bar {
        fn foo(&self) -> impl Iterator<Item = i32>;
    }
}

struct S;

impl base::Qux for S {
    fn qux(&self, x: i32) -> i32 {
        x
    }
}

impl base::Bar for S {
    async fn bar(&self, x: i32, _: u8) -> i32 {
        x + 1
    }
}

impl by_path::Foo for S {
    async fn foo(&self) -> i32 {
        3
    }
}

impl by_import::Foo for S {
    fn foo(&self) -> impl Iterator<Item = i32> {
        std::iter::once(4)
    }
}

async fn use_dyn(a: Box<by_path::DynFoo<'_>>, b: &by_import::DynFoo<'_>) -> i32 {
    use base::{Bar, Qux};
    use by_path::Foo as _;
    use by_import::Foo as _;
    a.qux(1) + a.bar(1, 0).await + a.foo().await + b.qux(1) + b.foo().sum::<i32>()
}

fn is_send<T: Send>(_: &T) {}

fn main() {
    let s = S;
    let a = by_path::DynFoo::new_box(S);
    is_send(&a);
    let _ = use_dyn(a, by_import::DynFoo::from_ref(&s));
}
