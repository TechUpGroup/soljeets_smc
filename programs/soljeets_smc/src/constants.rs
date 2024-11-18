// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;

#[constant]
pub const CONFIG_SEED: &[u8]  = b"jeets_config";
#[constant]
pub const VAULT: &[u8]  = b"vault";
#[constant]
pub const ATA_VAULT: &[u8]  = b"associate";
#[constant]
pub const MINT_AUTHORITY: &[u8]  = b"mint_authority";
#[constant]
pub const PDA_CHECK: &[u8]  = b"pda_buyer";
#[constant]
pub const PERCENT : u64 = 10000; // 