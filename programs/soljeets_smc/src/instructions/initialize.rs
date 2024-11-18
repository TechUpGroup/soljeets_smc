// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;

use crate::{Config, CONFIG_SEED, MINT_AUTHORITY};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no harm
    pub operator: UncheckedAccount<'info>,
    #[account(
        init,
        payer=authority,
        space = Config::LEN,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    /// CHECK:
    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [MINT_AUTHORITY],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_initialize(ctx: Context<Initialize>) -> Result<()> {
    let config_account = &mut ctx.accounts.config;
    config_account.decimal = 6;
    config_account.initial_token_reserve = 100_000_000_000 * 10u64.pow(6);
    config_account.operator = ctx.accounts.authority.key();
    config_account.operator_lp = ctx.accounts.operator.key();
    config_account.fee_receiver = ctx.accounts.authority.key();
    config_account.fee_platform = 50; // 0.5%
    config_account.fee_fund = 50; // 0.5%

    
    Ok(())
}
