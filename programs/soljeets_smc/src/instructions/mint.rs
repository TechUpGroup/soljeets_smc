use crate::error::ErrorMessage;
use crate::{Config, Vault, CONFIG_SEED};
use crate::{ATA_VAULT, MINT_AUTHORITY, VAULT};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer= payer,
        space = Vault::LEN,
        seeds = [
            VAULT, mint.key().as_ref()
        ],
        bump
    )]
    pub vault: Box<Account<'info, Vault>>,
    #[account(
        init,
        payer = payer,
        seeds=[
            ATA_VAULT,mint.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = vault
    )]
    pub ata_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: no harm,
    #[account(
        seeds=[
            MINT_AUTHORITY
        ],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: no harm, only use to check config
    pub operator: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler_mint(
    ctx: Context<MintToken>,
    creator: Pubkey,
    price: u64,
    sol_target: u64,
    max_buy: u64,
) -> Result<()> {
    require!(sol_target > 10_000_000_000, ErrorMessage::InvalidSolTarget);
    let config_account = &ctx.accounts.config;

    let seeds = &[&[
        MINT_AUTHORITY,
        bytemuck::bytes_of(&ctx.bumps.mint_authority),
    ][..]];
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.ata_vault.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);

    token::mint_to(cpi_ctx, config_account.initial_token_reserve)?;

    ctx.accounts.vault.mint = ctx.accounts.mint.key();
    ctx.accounts.vault.decimal = config_account.decimal;
    ctx.accounts.vault.token_supply = config_account.initial_token_reserve;
    ctx.accounts.vault.initial_token_reserve = config_account.initial_token_reserve;
    ctx.accounts.vault.initial_token_reserve = config_account.initial_token_reserve;
    ctx.accounts.vault.price = price;
    ctx.accounts.vault.sol_target = sol_target;
    ctx.accounts.vault.creator = creator;
    ctx.accounts.vault.max_token_buy = max_buy.checked_mul(1e6 as u64).unwrap();

    Ok(())
}
