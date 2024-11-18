// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use event::*;
pub use instructions::{
    buy::*, creator_buy::*, create_token::*, initialize::*, mint::*, sell::*, update_config::*, withdraw::*,
};
pub use state::*;

declare_id!("7Xp7mrXFKfUDsUYundLS6vHXRHtjFbsbuwQoVwkmVCMa");

#[program]
pub mod soljeets_smc {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handler_initialize(ctx)
    }

    pub fn create_token(
        ctx: Context<TokenInit>,
        name: String,
        ticker: String,
        uri: String,
        target_jeets_score: u16
    ) -> Result<()> {
        let (token, creator, total_supply) =
            handler_create(ctx, name, ticker, uri).unwrap();
        emit!(TokenCreatedEvent {
            token,
            creator,
            total_supply,
            target_jeets_score
        });
        Ok(())
    }

    pub fn mint(ctx: Context<MintToken>, creator: Pubkey,price: u64, sol_target: u128, max_buy: u64) -> Result<()> {
        handler_mint(ctx,creator,price, sol_target, max_buy)
    }

    pub fn buy(ctx: Context<Buy>, amount_sol: u64) -> Result<()> {
        let (
            token,
            account,
            virtual_sol_reserve,
            virtual_token_reserve,
            amount_sol,
            amount_token,
            completed,
        ) = handler_buy(ctx, amount_sol).unwrap();
        emit!(TradingEvent {
            token,
            account,
            amount_sol,
            amount_token,
            is_buy: true,
            virtual_sol_reserve,
            virtual_token_reserve,
            completed
        });
        Ok(())
    }

    pub fn creator_buy(ctx: Context<CreatorBuy>) -> Result<()> {
        let (
            token,
            account,
            virtual_sol_reserve,
            virtual_token_reserve,
            amount_sol,
            amount_token,
            completed,
        ) = handler_creator_buy(ctx).unwrap();
        emit!(TradingEvent {
            token,
            account,
            amount_sol,
            amount_token,
            is_buy: true,
            virtual_sol_reserve,
            virtual_token_reserve,
            completed
        });
        Ok(())
    }

    pub fn sell(ctx: Context<Sell>, amount_token: u64) -> Result<()> {
        let (token, account, virtual_sol_reserve, virtual_token_reserve, amount_sol, amount_token) =
            handler_sell(ctx, amount_token).unwrap();
        emit!(TradingEvent {
            token,
            account,
            amount_sol,
            amount_token,
            is_buy: false,
            virtual_sol_reserve,
            virtual_token_reserve,
            completed: false
        });
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let (token, account, _, _, amount_sol, amount_token) = handler_withdraw(ctx).unwrap();
        emit!(WithdrawEvent {
            token,
            account,
            amount_sol,
            amount_token
        });
        Ok(())
    }

    pub fn update_config(
        ctx: Context<Update>,
        initial_token_reserve: Option<u64>,
        fee_platform: Option<u16>,
        fee_fund: Option<u16>,
        fund: Option<Pubkey>,
        new_operator_lp: Option<Pubkey>,
        new_fee_receiver: Option<Pubkey>,
        new_operator: Option<Pubkey>
    ) -> Result<()> {
        handle_update(
            ctx,
            initial_token_reserve,
            fee_platform,
            fee_fund,
            fund,
            new_operator_lp,
            new_fee_receiver,
            new_operator
        )
    }
}
