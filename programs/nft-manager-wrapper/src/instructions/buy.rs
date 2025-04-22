use crate::state::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{burn, Burn};
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3,
            Metadata,
        },
        token::{approve, mint_to, transfer, Approve, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(src_nft: Pubkey, token_type: u8)]
pub struct BuyCallContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"price",
            src_nft.as_ref(),
            &token_type.to_le_bytes(),
        ],
        bump,
    )]
    pub price_account: Account<'info, Price>,

    #[account(mut)]
    pub recv_sol: AccountInfo<'info>,

    // spl代币转账相关账户
    #[account(mut)]
    pub sender_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recv_ata: Account<'info, TokenAccount>,
    
    // nft mint
    pub mint_auth: AccountInfo<'info>,
    #[account(mut)]
    pub mint_ata: Account<'info, Mint>,
    #[account(mut)]
    pub sender_nft_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recv_nft_ata: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy(
    ctx: Context<BuyCallContext>,
    _src_nft: Pubkey,
    _token_type: u8,
    value: u64, // token数量
) -> Result<()> {
    let price_info = &mut ctx.accounts.price_account;

    let amount = value
        .checked_mul(price_info.price)
        .ok_or(NftManageWrapperError::MathOverflow)?;
    // SOL
    if price_info.token_type == 1 {
        invoke_signed(
            &system_instruction::transfer(
                ctx.accounts.sender.key,
                ctx.accounts.recv_sol.key,
                amount,
            ),
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.recv_sol.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
    } else {
        // SPL
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sender_ata.to_account_info(),
                    to: ctx.accounts.recv_ata.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            amount,
        )?;
    }
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_ata.to_account_info(),
                to: ctx.accounts.sender_nft_ata.to_account_info(),
                authority: ctx.accounts.mint_auth.to_account_info(),
            },
        ),
        value,
    )?;
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_nft_ata.to_account_info(),
                to: ctx.accounts.recv_nft_ata.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        value,
    )?;
    burn(
        CpiContext::new(
          ctx.accounts.token_program.to_account_info(),
          Burn {
              mint: ctx.accounts.mint_ata.to_account_info(),
              from: ctx.accounts.recv_nft_ata.to_account_info(),
              authority: ctx.accounts.recv_sol.to_account_info(),
          }
        ),
        value,
    )?;
    
    // msg!("buy: {}, {}, {}, {}, {}, {}, {}", ctx.accounts.sender.key, value, );
    Ok(())
}
