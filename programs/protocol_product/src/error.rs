use anchor_lang::prelude::*;

#[error_code]
pub enum ProductError {
    #[msg("Product: commission rate must be >= 0 and <= 100.")]
    InvalidCommissionRate,
    #[msg("Product: product title must not be empty.")]
    ProductTitleEmpty,
    #[msg("Product: Commission supports up to 3 decimal places.")]
    CommissionPrecisionTooLarge,
}
