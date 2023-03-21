use crate::state::product::Product;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

#[derive(Accounts)]
#[instruction(product_title: String)]
pub struct CreateProduct<'info> {
    #[account(
        init,
        seeds = [b"product".as_ref(), product_title.as_ref()],
        bump,
        payer = payer,
        space = Product::SIZE
    )]
    pub product: Account<'info, Product>,
    pub commission_escrow: SystemAccount<'info>,

    // payer may differ from authority in the case of multisig pda as authority
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_product_title: String)]
pub struct UpdateProduct<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"product".as_ref(), _product_title.as_ref()],
        bump,
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(_product_title: String)]
pub struct UpdateProductAuthority<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"product".as_ref(), _product_title.as_ref()],
        bump,
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub updated_authority: Signer<'info>,
}
