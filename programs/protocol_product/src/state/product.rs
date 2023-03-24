use crate::state::type_size::{vec_size, CHAR_SIZE, DISCRIMINATOR_SIZE, F64_SIZE, PUB_KEY_SIZE};
use anchor_lang::prelude::*;

#[account]
pub struct Product {
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub commission_escrow: Pubkey,
    pub product_title: String,
    pub commission_rate: f64,
}

impl Product {
    pub const PRODUCT_TITLE_MAX_LENGTH: usize = 32;
    pub const SIZE: usize = DISCRIMINATOR_SIZE +
        (PUB_KEY_SIZE * 3) + // authority, payer and commission_escrow
        vec_size (CHAR_SIZE, Product::PRODUCT_TITLE_MAX_LENGTH) + // product_title
        F64_SIZE; // commission rate
}
