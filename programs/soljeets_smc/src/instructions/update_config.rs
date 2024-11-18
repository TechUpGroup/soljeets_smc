// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;

use crate::{Config, CONFIG_SEED};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
}

pub fn handle_update(
    ctx: Context<Update>,
    initial_token_reserve: Option<u64>,
    fee_platform: Option<u16>,
    fee_fund: Option<u16>,
    fund: Option<Pubkey>,
    new_operator_lp: Option<Pubkey>,
    new_fee_receiver: Option<Pubkey>,
    new_operator: Option<Pubkey>,
) -> Result<()> {
    let config_account = &mut ctx.accounts.config;

    if let Some(new_operator) = new_operator {
        config_account.operator = new_operator;
    }

    if let Some(new_operator_lp) = new_operator_lp {
        config_account.operator_lp = new_operator_lp;
    }

    if let Some(new_fee_receiver) = new_fee_receiver {
        config_account.fee_receiver = new_fee_receiver;
    }

    if let Some(fund) = fund {
        config_account.fund = fund;
    }

    if let Some(fee_fund) = fee_fund {
        config_account.fee_fund = fee_fund;
    }

    if let Some(fee_platform) = fee_platform {
        config_account.fee_platform = fee_platform;
    }

    if let Some(initial_token_reserve) = initial_token_reserve {
        config_account.initial_token_reserve = initial_token_reserve;
    }

    Ok(())
}
