// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use crate::error::ErrorMessage;
use crate::utils::transfer_token_to_account;
use crate::{Config, CONFIG_SEED};
use crate::{Vault, ATA_VAULT, VAULT};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub operator_lp: Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump,
        has_one = operator_lp
    )]
    pub config: Account<'info, Config>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            VAULT, mint.key().as_ref()
        ],
        bump,
        constraint= vault.completed == true @ErrorMessage::TradingNotEnd
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
        init_if_needed,
        payer = operator_lp,
        associated_token::mint = mint,
        associated_token::authority=operator_lp,
    )]
    pub associate_operator: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = operator_lp,
        associated_token::mint = mint,
        associated_token::authority=dev,
    )]
    pub associate_dev: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub dev: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler_withdraw(ctx: Context<Withdraw>) -> Result<(Pubkey, Pubkey, u64, u64, u64, u64)> {
    // calculate token receive
    let vault = &mut ctx.accounts.vault;

    let binding = ctx.accounts.mint.key();
    let seeds = &[&[
        VAULT,
        binding.as_ref(),
        bytemuck::bytes_of(&ctx.bumps.vault),
    ][..]];
    let amount_token = ctx.accounts.ata_vault.amount;
    let amount_token_lp = amount_token
        .checked_mul(90)
        .unwrap()
        .checked_div(100)
        .unwrap();

    transfer_token_to_account(
        ctx.accounts.ata_vault.to_account_info(),
        ctx.accounts.associate_operator.to_account_info(),
        vault.to_account_info(),
        amount_token_lp,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds),
    )?;

    // transfer 10% token to creator to distribute
    transfer_token_to_account(
        ctx.accounts.ata_vault.to_account_info(),
        ctx.accounts.associate_dev.to_account_info(),
        vault.to_account_info(),
        amount_token.checked_sub(amount_token_lp).unwrap(),
        ctx.accounts.token_program.to_account_info(),
        Some(seeds),
    )?;

    let amount_sol = vault.get_lamports() - 1746960u64;
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount_sol + 1746960u64;
    let amount_sol_lp = amount_sol
    .checked_mul(90)
    .unwrap()
    .checked_div(100)
    .unwrap();
    **ctx
        .accounts
        .operator_lp
        .to_account_info()
        .try_borrow_mut_lamports()? += amount_sol_lp;
    **ctx
        .accounts
        .dev
        .to_account_info()
        .try_borrow_mut_lamports()? += amount_sol
        .checked_mul(10)
        .unwrap()
        .checked_div(100)
        .unwrap()
        + 1746960u64;

    Ok((
        ctx.accounts.mint.key(),
        ctx.accounts.operator_lp.key(),
        0,
        0,
        amount_sol_lp,
        amount_token_lp,
    ))
}
