mod instructions;
mod state;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("5Pz73eRmegxqtFDA2zjUGT6iYPxfMrmPEbDu7NcYSKGy");

#[program]
pub mod nft_manager_wrapper {
    use super::*;

    pub fn set_price(
        ctx: Context<SetPriceContext>,
        src_nft: Pubkey,
        token_type: u8,
        price: u64,
    ) -> Result<()> {
        prices::set_price(ctx, src_nft, token_type, price)
    }

    pub fn buy(
        ctx: Context<BuyCallContext>,
        mint_nft: Pubkey,
        token_type: u8,
        value: u64,
    ) -> Result<()> {
        buy::buy(ctx, mint_nft, token_type, value)
    }
}
