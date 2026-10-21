//! Regression test: an attribute macro that synthesizes `self` (here
//! `mockall::automock`) must see the receivers of methods carried from a
//! `#[dynosaur]` supertrait. Those receivers travel through the callback
//! `macro_rules!`, so without re-spanning them the generated mock bodies
//! fail with "cannot find value `self` in this scope".

use dynosaur::dynosaur;

#[dynosaur(DynStore = dyn(box) Store)]
#[mockall::automock]
trait Store {
    fn key(&self) -> &'static str;
}

#[dynosaur(DynCustomerStore = dyn(box) CustomerStore)]
#[mockall::automock]
trait CustomerStore: Store {
    fn find(&self, id: u32) -> Option<String>;
}

impl Store for MockCustomerStore {
    fn key(&self) -> &'static str {
        "customer"
    }
}

#[test]
fn mocked_subtrait_erases() {
    let mut mock = MockCustomerStore::new();
    mock.expect_find().return_const(Some("x".to_string()));
    let dyn_store = DynCustomerStore::from_ref(&mock);
    assert_eq!(dyn_store.key(), "customer");
    assert_eq!(dyn_store.find(1), Some("x".to_string()));
}
