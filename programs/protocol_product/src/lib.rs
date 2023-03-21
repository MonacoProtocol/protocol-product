use anchor_lang::prelude::*;

use crate::context::*;

pub mod context;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("mppFrYmM6A4Ud3AxRbGXsGisX1HUsbDfp1nrg9FQJEE");

#[program]
pub mod protocol_product {
    use super::*;

    pub fn create_product(
        ctx: Context<CreateProduct>,
        product_title: String,
        commission_rate: f64,
    ) -> Result<()> {
        instructions::create_product(
            &mut ctx.accounts.product,
            &ctx.accounts.authority.key(),
            &ctx.accounts.payer.key(),
            product_title,
            commission_rate,
            &ctx.accounts.commission_escrow.key(),
        )
    }

    pub fn update_product_commission_escrow(
        ctx: Context<UpdateProduct>,
        _product_title: String,
        updated_commission_escrow: Pubkey,
    ) -> Result<()> {
        instructions::update_product_commission_escrow(
            &mut ctx.accounts.product,
            updated_commission_escrow,
        )
    }

    pub fn update_product_commission_rate(
        ctx: Context<UpdateProduct>,
        _product_title: String,
        updated_commission_rate: f64,
    ) -> Result<()> {
        instructions::update_product_commission_rate(
            &mut ctx.accounts.product,
            updated_commission_rate,
        )
    }

    pub fn update_product_authority(
        ctx: Context<UpdateProductAuthority>,
        _product_title: String,
    ) -> Result<()> {
        instructions::update_product_authority(
            &mut ctx.accounts.product,
            ctx.accounts.updated_authority.key(),
        )
    }
}
