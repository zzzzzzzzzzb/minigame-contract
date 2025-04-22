use anchor_lang::prelude::*;

#[account]
pub struct Price {
    pub src_nft: Pubkey,
    pub price: u64,
    pub token_type: u8,
}
