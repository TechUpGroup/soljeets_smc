// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub initial_token_reserve: u64,
    pub decimal: u8,
    pub operator: Pubkey,
    pub operator_lp: Pubkey,
    pub fund: Pubkey,
    pub fee_receiver: Pubkey,
    pub fee_platform: u16,
    pub fee_fund: u16,
}

impl Config {
    pub const LEN: usize = 8 + 1 * 1 + 2 * 2 + 1 * 8 + 4 * 32;
}
