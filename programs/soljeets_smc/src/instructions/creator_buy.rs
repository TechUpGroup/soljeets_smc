// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use crate::error::ErrorMessage;
use crate::utils::
    transfer_token_to_account
;
use crate::{Config, Vault, ATA_VAULT, CONFIG_SEED, VAULT};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreatorBuy<'info> {
    #[account(
        mut
    )]
    pub buyer: Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump,
        has_one= fund,
        has_one= fee_receiver
    )]
    pub config: Account<'info, Config>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
           VAULT , mint.key().as_ref(), 
        ],
        bump,
        constraint = vault.completed == false @ ErrorMessage::TradingEnd,
        constraint = vault.creator == buyer.key() @ErrorMessage::InvalidCreator,
        constraint = vault.creator_buy == false @ErrorMessage::BuyOnce,
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
    pub associate_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub associate_user: Box<Account<'info, TokenAccount>>,
    /// CHECK: receive fee platform
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    /// CHECK: receive fund
    #[account(mut)]
    pub fund: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler_creator_buy(
    ctx: Context<CreatorBuy>
) -> Result<(Pubkey, Pubkey, u128, u64, u64, u64, bool)> {
    let vault= &mut ctx.accounts.vault;
    let current_total_sol = vault.get_lamports() as u128- 1746960u128;

    // calculate token receive
    let token_reserve = ctx.accounts.associate_vault.amount;

    let amount_token_out = (vault.initial_token_reserve as u128).checked_div(100 as u128).unwrap() as u64; // 1% for creator, no fee
    let remaining_token = token_reserve.checked_sub(amount_token_out).unwrap();
   
    require!(
        amount_token_out > 0 && amount_token_out as u64 <= token_reserve,
        ErrorMessage::InvalidAmountToken
    );

    // tranfer token to buyer
    let binding = ctx.accounts.mint.key();
    let seeds = &[&[
        VAULT,
        binding.as_ref(),
        bytemuck::bytes_of(&ctx.bumps.vault),
    ][..]];

    transfer_token_to_account(
        ctx.accounts.associate_vault.to_account_info(),
        ctx.accounts.associate_user.to_account_info(),
        vault.to_account_info(),
        amount_token_out as u64,
        ctx.accounts.token_program.to_account_info(),
        Some(seeds),
    )?;

    if current_total_sol.eq(&(vault.sol_target)) {
        vault.completed = true;
    }

    vault.creator_buy = true;

    Ok((
        ctx.accounts.mint.key(),
        ctx.accounts.buyer.key(),
        current_total_sol,
        remaining_token,
        0u64,
        amount_token_out as u64,
        vault.completed,
    ))
}