use crate::faker::numerify_sym_with_digits;
use crate::faker::phone_number::raw::*;
use crate::faker::replace_ascii_digits_with_locale;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::IndexedRandom;
use rand::RngExt;

impl<L: Data> Dummy<PhoneNumber<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &PhoneNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_NUMBER_FORMATS.choose(rng).unwrap();
        let s = numerify_sym_with_digits(fmt, rng, L::NUMBER_DIGIT);
        replace_ascii_digits_with_locale(&s, L::NUMBER_DIGIT)
    }
}

impl<L: Data> Dummy<CellNumber<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_c: &CellNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_CELL_NUMBER_FORMATS.choose(rng).unwrap();
        let s = numerify_sym_with_digits(fmt, rng, L::NUMBER_DIGIT);
        replace_ascii_digits_with_locale(&s, L::NUMBER_DIGIT)
    }
}
