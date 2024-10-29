use crate::error::ErrorMessage;
use crate::utils::{calculate_fee, transfer_token_to_account};
use crate::{Config, Vault, ATA_VAULT, CONFIG_SEED, VAULT};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(max_amount_token: u64)]
pub struct Sell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump,
        has_one=fund,
        has_one=fee_receiver
    )]
    pub config: Account<'info, Config>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            VAULT, mint.key().as_ref()
        ],
        bump,
        constraint = vault.completed == false @ ErrorMessage::TradingEnd
    )]
    pub vault: Box<Account<'info, Vault>>,
    #[account(
        mut,
        seeds=[
            ATA_VAULT,mint.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = vault
    )]
    pub ata_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        constraint = associate_user.amount >= max_amount_token @ErrorMessage::InsufficientToken
    )]
    pub associate_user: Box<Account<'info, TokenAccount>>,
    /// CHECK: receive fee platform
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    /// CHECK: receive fee fund
    #[account(mut)]
    pub fund: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler_sell(
    ctx: Context<Sell>,
    max_amount_token: u64,
) -> Result<(Pubkey, Pubkey, u64, u64, u64, u64)> {
    let config_account = &ctx.accounts.config;

    // calculate token receive
    let vault = &mut ctx.accounts.vault;
    let price = vault.price;
    let mut current_total_sol = vault.get_lamports() - 1746960u64;
    let mut current_total_token = ctx.accounts.ata_vault.amount;
    let amount_sol_out = (max_amount_token as u128)
        .checked_mul(price as u128)
        .unwrap()
        .checked_div(1e6 as u128)
        .unwrap() as u64;

    require!(
        amount_sol_out.gt(&0) && amount_sol_out.le(&current_total_sol),
        ErrorMessage::InvalidAmountSolTrade
    );

    let (amount_sol, _fee_platform, fee_fund) = calculate_fee(
        amount_sol_out,
        0,
        config_account.fee_fund
    )
    .unwrap();

    require!(amount_sol > 0, ErrorMessage::InvalidAmountSol);
    // tranfer token to ata vault
    transfer_token_to_account(
        ctx.accounts.associate_user.to_account_info(),
        ctx.accounts.ata_vault.to_account_info(),
        ctx.accounts.seller.to_account_info(),
        max_amount_token,
        ctx.accounts.token_program.to_account_info(),
        None,
    )?;
    current_total_token = current_total_token.checked_add(max_amount_token).unwrap();

    **vault.to_account_info().try_borrow_mut_lamports()? -= amount_sol_out;
    **ctx
        .accounts
        .fund
        .to_account_info()
        .try_borrow_mut_lamports()? += fee_fund;
    **ctx
        .accounts
        .seller
        .to_account_info()
        .try_borrow_mut_lamports()? += amount_sol_out - fee_fund;
    current_total_sol = current_total_sol.checked_sub(amount_sol_out).unwrap();

    Ok((
        ctx.accounts.mint.key(),
        ctx.accounts.seller.key(),
        current_total_sol,
        current_total_token,
        amount_sol_out,
        max_amount_token,
    ))
}
