//! Print total count of COMMERCE* array entries for BN_BD locale.
//! Run with: cargo run --example commerce_counts

use fake::locales::Data;
use fake::locales::BN_BD;

fn main() {
    let color = BN_BD::COMMERCE_COLOR.len();
    let department = BN_BD::COMMERCE_DEPARTMENT.len();
    let adjective = BN_BD::COMMERCE_PRODUCT_ADJECTIVE.len();
    let material = BN_BD::COMMERCE_PRODUCT_MATERIAL.len();
    let product_type = BN_BD::COMMERCE_PRODUCT_TYPE.len();
    let prefix = BN_BD::COMMERCE_PROMOTION_CODE_PREFIX.len();
    let suffix = BN_BD::COMMERCE_PROMOTION_CODE_SUFFIX.len();
    let description = BN_BD::COMMERCE_PRODUCT_DESCRIPTION.len();

    println!("BN_BD COMMERCE array counts (at least 500 per type):");
    println!("  COMMERCE_COLOR:                 {}", color);
    println!("  COMMERCE_DEPARTMENT:            {}", department);
    println!("  COMMERCE_PRODUCT_ADJECTIVE:     {}", adjective);
    println!("  COMMERCE_PRODUCT_MATERIAL:      {}", material);
    println!("  COMMERCE_PRODUCT_TYPE:          {}", product_type);
    println!("  COMMERCE_PROMOTION_CODE_PREFIX: {}", prefix);
    println!("  COMMERCE_PROMOTION_CODE_SUFFIX: {}", suffix);
    println!("  COMMERCE_PRODUCT_DESCRIPTION:   {}", description);

    let total: usize =
        color + department + adjective + material + product_type + prefix + suffix + description;
    println!("  ----------------------------------------");
    println!("  TOTAL:                          {}", total);

    let all_500 = [
        color,
        department,
        adjective,
        material,
        product_type,
        prefix,
        suffix,
        description,
    ]
    .iter()
    .all(|&n| n >= 500);
    if all_500 {
        println!("\n✓ All COMMERCE types have at least 500 entries.");
    } else {
        println!("\n✗ Some types have fewer than 500 entries.");
    }
}
