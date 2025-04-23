use crate::state::*;
use {
    anchor_lang::{
        prelude::*,
        solana_program::{program::invoke_signed, system_instruction},
    },
    anchor_spl::token::{
        burn, mint_to, transfer, Burn, Mint, MintTo, Token, TokenAccount, Transfer,
    },
};

#[derive(Accounts)]
#[instruction(mint_nft: Pubkey, token_type: u8)]
pub struct BuyCallContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"price",
            mint_nft.as_ref(),
            &token_type.to_le_bytes(),
        ],
        bump,
    )]
    pub price_account: Account<'info, Price>,

    #[account(mut)]
    pub owner: AccountInfo<'info>,

    // spl代币转账相关账户
    #[account(mut)]
    pub sender_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_ata: Account<'info, TokenAccount>,

    // nft mint
    #[account(mut)]
    pub mint_ata: Account<'info, Mint>,
    #[account(mut)]
    pub sender_nft_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_nft_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BuyEvent {
    pub user: Pubkey,
    pub mint_nft: Pubkey,
    pub token_type: u8,
    pub value: u64,
}

pub fn buy(
    ctx: Context<BuyCallContext>,
    _mint_nft: Pubkey,
    _token_type: u8,
    value: u64, // token数量
) -> Result<()> {
    let price_info = &mut ctx.accounts.price_account;

    require!(
        ctx.accounts.mint_ata.key() == price_info.mint_nft,
        NftManageWrapperError::MintAccountMismatch
    );

    let amount = value
        .checked_mul(price_info.price)
        .ok_or(NftManageWrapperError::MathOverflow)?;
    // SOL
    if price_info.token_type == 1 {
        invoke_signed(
            &system_instruction::transfer(ctx.accounts.sender.key, ctx.accounts.owner.key, amount),
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.owner.to_account_info(),
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
                    to: ctx.accounts.owner_ata.to_account_info(),
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
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        value,
    )?;
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_nft_ata.to_account_info(),
                to: ctx.accounts.owner_nft_ata.to_account_info(),
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
                from: ctx.accounts.owner_nft_ata.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        value,
    )?;

    emit!(BuyEvent {
        user: ctx.accounts.sender.key(),
        mint_nft: ctx.accounts.mint_ata.key(),
        token_type: price_info.token_type,
        value,
    });
    Ok(())
}
