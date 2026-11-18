#![cfg(feature = "nonempty")]

use fake::{Dummy, Fake, Faker};
use nonempty::NonEmpty;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn faker_gives_one_to_ten_elements() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut seen = [false; 11];
    for _ in 0..1000 {
        let v: NonEmpty<u8> = Faker.fake_with_rng(&mut rng);
        seen[v.len()] = true;
    }
    assert!(!seen[0]);
    assert!(seen[1..].iter().all(|s| *s), "{seen:?}");
}

#[test]
fn length_config_is_the_total_length() {
    let v: NonEmpty<u8> = (Faker, 3).fake();
    assert_eq!(v.len(), 3);

    let v: NonEmpty<u8> = (Faker, 1).fake();
    assert_eq!(v.len(), 1);

    for _ in 0..100 {
        let v: NonEmpty<u8> = (Faker, 2..5).fake();
        assert!((2..5).contains(&v.len()), "{}", v.len());
    }
}

#[test]
fn zero_length_gives_one_element() {
    let v: NonEmpty<u8> = (Faker, 0).fake();
    assert_eq!(v.len(), 1);
}

#[test]
fn element_config_applies_to_head_and_tail() {
    let v: NonEmpty<u8> = (10..20, 8).fake();
    assert_eq!(v.len(), 8);
    assert!(v.iter().all(|n| (10..20).contains(n)), "{v:?}");
}

#[test]
fn derive_supports_nonempty_fields() {
    #[derive(Debug, Dummy)]
    struct Order {
        lines: NonEmpty<String>,
        #[dummy(faker = "(1..5, 2..4)")]
        quantities: NonEmpty<u32>,
    }

    let order: Order = Faker.fake();
    assert!(!order.lines.is_empty());
    assert!((2..4).contains(&order.quantities.len()));
    assert!(order.quantities.iter().all(|q| (1..5).contains(q)));
}
