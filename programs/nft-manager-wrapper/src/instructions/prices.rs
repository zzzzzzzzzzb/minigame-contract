use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(mint_nft: Pubkey, token_type: u8)]
pub struct SetPriceContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32+ 8 + 1 + 1,
        seeds = [
            b"price",
            mint_nft.as_ref(),
            &token_type.to_le_bytes(),
        ],
        bump,
    )]
    pub price_account: Account<'info, Price>,

    pub system_program: Program<'info, System>,
}

pub fn set_price(ctx: Context<SetPriceContext>, mint_nft: Pubkey, token_type: u8, price: u64) -> Result<()> {
    *ctx.accounts.price_account = Price {
        mint_nft,
        price,
        token_type,
    };
    msg!("set price {}", price);
    Ok(())
}