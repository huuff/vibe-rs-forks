use dynosaur::dynosaur;

#[dynosaur(DynBar = dyn(box) Bar)]
trait Bar<T> {
    fn bar(&self) -> T;
}

#[dynosaur(DynFoo = dyn(box) Foo)]
trait Foo: Bar<i32> {
    async fn foo(&self) -> i32;
}

fn main() {}
