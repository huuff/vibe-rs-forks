use crate::faker::commerce::raw::*;
use crate::faker::numerify_sym_with_digits;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::IndexedRandom;
use rand::RngExt;

impl<L: Data> Dummy<CommerceColor<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceColor<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_COLOR.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceDepartment<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceDepartment<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_DEPARTMENT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductAdjective<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceProductAdjective<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductMaterial<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceProductMaterial<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductType<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceProductType<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProduct<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceProduct<L>, rng: &mut R) -> Self {
        let fmt = L::COMMERCE_PRODUCT;
        let adjective = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
        let material = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
        let product = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
        fmt.replace("{Adjective}", adjective)
            .replace("{Material}", material)
            .replace("{Product}", product)
    }
}

impl<L: Data> Dummy<CommerceProductPrice<L>> for f64 {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &CommerceProductPrice<L>, rng: &mut R) -> Self {
        let range = c.1.clone();
        let price = rng.random_range(range);
        (price * 100.0).round() / 100.0
    }
}

impl<L: Data> Dummy<CommerceProductPrice<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &CommerceProductPrice<L>, rng: &mut R) -> Self {
        let range = c.1.clone();
        let price = rng.random_range(range);
        format!("{:.2}", (price * 100.0).round() / 100.0)
    }
}

impl<L: Data> Dummy<CommercePromotionCode<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommercePromotionCode<L>, rng: &mut R) -> Self {
        let fmt = *L::COMMERCE_PROMOTION_CODE.choose(rng).unwrap();
        let prefix = *L::COMMERCE_PROMOTION_CODE_PREFIX.choose(rng).unwrap();
        let suffix = *L::COMMERCE_PROMOTION_CODE_SUFFIX.choose(rng).unwrap();
        let prefix = numerify_sym_with_digits(prefix, rng, L::NUMBER_DIGIT);
        let suffix = numerify_sym_with_digits(suffix, rng, L::NUMBER_DIGIT);
        fmt.replace("{Prefix}", &prefix)
            .replace("{Suffix}", &suffix)
    }
}

impl<L: Data> Dummy<CommerceProductDescription<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceProductDescription<L>, rng: &mut R) -> Self {
        // If you added COMMERCE_PRODUCT_DESCRIPTION to locales/mod.rs, use this:
        if !L::COMMERCE_PRODUCT_DESCRIPTION.is_empty() {
            let fmt = *L::COMMERCE_PRODUCT_DESCRIPTION.choose(rng).unwrap();
            let adjective = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
            let material = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
            let product = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
            let department = *L::COMMERCE_DEPARTMENT.choose(rng).unwrap();

            return fmt
                .replace("{Adjective}", adjective)
                .replace("{Material}", material)
                .replace("{Product}", product)
                .replace("{Department}", department);
        }

        // Fallback logic if the description list is missing
        let adj1 = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
        let mat = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
        // ERROR WAS HERE: Changed COMMERCE_PRODUCT_NAME to COMMERCE_PRODUCT_TYPE
        let prod = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
        let adj2 = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();

        format!(
            "The {} {} {} is designed for {} usage.",
            adj1, mat, prod, adj2
        )
    }
}

impl<L: Data> Dummy<CommerceUPC<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CommerceUPC<L>, rng: &mut R) -> Self {
        // UPC-A: 11 digits + 1 checksum
        let mut digits: Vec<u32> = (0..11).map(|_| rng.random_range(0..10)).collect();

        // Checksum: (Sum of odd-position digits * 3) + (Sum of even-position digits)
        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| if (i + 1) % 2 == 1 { d * 3 } else { d })
            .sum();

        let remainder = sum % 10;
        let check_digit = if remainder == 0 { 0 } else { 10 - remainder };

        digits.push(check_digit);
        digits.iter().map(|d| d.to_string()).collect()
    }
}
