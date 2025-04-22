use anchor_lang::prelude::*;

#[error_code]
pub enum NftManageWrapperError {
    #[msg("NftManageWrapperError: Math overflow")]
    MathOverflow,
    #[msg("NftManageWrapperError: MintAccount Mismatch")]
    MintAccountMismatch,
}