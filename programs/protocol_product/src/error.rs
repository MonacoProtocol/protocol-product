use anchor_lang::prelude::*;

#[error_code]
pub enum ProductError {
    #[msg("Product: commission rate must be >= 0 and <= 100.")]
    InvalidCommissionRate,
    #[msg("Product: title length must be between 1 and 50 characters.")]
    ProductTitleLen,
    #[msg("Product: Commission supports up to 3 decimal places.")]
    CommissionPrecisionTooLarge,
}
