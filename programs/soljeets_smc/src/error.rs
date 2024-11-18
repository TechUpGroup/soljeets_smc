// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorMessage {
    #[msg("Insufficient fund")]
    InsufficientFund,
    #[msg("Invalid amount sol")]
    InvalidAmountSol,
    #[msg("Insufficient token")]
    InsufficientToken,
    #[msg("Trading: completed, cannot buy")]
    TradingEnd,
    #[msg("Trading: not completed")]
    TradingNotEnd,
    #[msg("Trading: Invalid amount of token available to trade")]
    InvalidAmountToken,
    #[msg("Trading: Invalid amount of sol available to trade")]
    InvalidAmountSolTrade,
    #[msg("Withdraw: Creator invalid")]
    InvalidCreator,
    #[msg("Buy: Exceed max token can hold")]
    ExceedMaximum,
    #[msg("Creator 1st buy: already bought")]
    BuyOnce,
    #[msg("Sol target must be larger than 5 Sol")]
    InvalidSolTarget,
    #[msg("Invalid PDA")]
    InvalidPda,
}
