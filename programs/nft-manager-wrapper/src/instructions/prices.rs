use crate::state::*;
use anchor_lang::prelude::*;

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

#[event]
pub struct SetPriceEvent {
    pub mint_nft: Pubkey,
    pub token_type: u8,
    pub price: u64,
}

pub fn set_price(
    ctx: Context<SetPriceContext>,
    mint_nft: Pubkey,
    token_type: u8,
    price: u64,
) -> Result<()> {
    *ctx.accounts.price_account = Price {
        mint_nft,
        price,
        token_type,
    };

    emit!(SetPriceEvent {
        mint_nft,
        token_type,
        price
    });

    Ok(())
}
