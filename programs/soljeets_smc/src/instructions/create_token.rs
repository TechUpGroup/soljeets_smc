// SPDX-License-Identifier: BUSL-1.1
//
// Copyright (C) 2024 [SolJeets]
// 
// This file is licensed under the Business Source License 1.1. Details can be found in the LICENSE file.

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};
use mpl_token_metadata::types::DataV2;
use crate::MINT_AUTHORITY;
use crate::{Config, CONFIG_SEED};

#[derive(Accounts)]
#[instruction(name: String,ticker: String, uri: String)]
pub struct TokenInit<'info> {
    #[account(
        mut
    )]
    pub payer : Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump
    )]
    pub config : Account<'info,Config>,
    #[account(
        init, 
        payer = payer,
        mint::decimals = config.decimal,
        mint::authority = mint_authority,
        mint::token_program = token_program
    )]
    pub mint: Box<Account<'info,Mint>>,
    /// CHECK: no harm
    #[account(
        seeds=[
            MINT_AUTHORITY
        ],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: no harm, only use to check config
    pub operator: UncheckedAccount<'info>,
    /// CHECK: no harm
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: AccountInfo<'info>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
     /// CHECK: no harm
    pub token_metadata_program: AccountInfo<'info>,
    /// CHECK: no harm
    pub rent: AccountInfo<'info>,
}

pub fn handler_create(ctx: Context<TokenInit>, name: String,ticker: String, uri: String) -> Result<(Pubkey, Pubkey,u64)> {

    let seeds = &[&[
        MINT_AUTHORITY,
        bytemuck::bytes_of(&ctx.bumps.mint_authority),
    ][..]];
   

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.mint_authority.to_account_info(),
                update_authority: ctx.accounts.mint_authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            seeds
        ),
        DataV2 {
            name,
            symbol: ticker,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    Ok((ctx.accounts.mint.key(),ctx.accounts.payer.key(),ctx.accounts.config.initial_token_reserve))
}
