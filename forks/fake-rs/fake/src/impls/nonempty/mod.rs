use crate::{Dummy, Fake, Faker};
use nonempty::NonEmpty;
use rand::RngExt;

impl<T> Dummy<Faker> for NonEmpty<T>
where
    T: Dummy<Faker>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        NonEmpty {
            head: config.fake_with_rng(rng),
            tail: config.fake_with_rng(rng),
        }
    }
}

/// The length is the total length, head included; a length of 0 gives 1.
impl<T, E, L> Dummy<(E, L)> for NonEmpty<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &(E, L), rng: &mut R) -> Self {
        let len: usize = config.1.fake_with_rng(rng);
        let head = config.0.fake_with_rng(rng);
        let mut tail = Vec::with_capacity(len.saturating_sub(1));
        for _ in 1..len {
            tail.push(config.0.fake_with_rng(rng));
        }
        NonEmpty { head, tail }
    }
}
