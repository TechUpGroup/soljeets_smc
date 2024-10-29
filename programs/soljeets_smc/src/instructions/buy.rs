use crate::error::ErrorMessage;
use crate::utils::{
    calculate_fee, transfer_native_to_account, transfer_token_to_account
};
use crate::{Config, Vault, ATA_VAULT, CONFIG_SEED, PDA_CHECK, VAULT};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(max_amount_sol: u64)]
pub struct Buy<'info> {
    #[account(
        mut,
        constraint = buyer.lamports() >= max_amount_sol @ ErrorMessage::InsufficientFund
    )]
    pub buyer: Signer<'info>,
    #[account(
        seeds=[CONFIG_SEED],
        bump,
        has_one= fund,
        has_one=operator
    )]
    pub config: Account<'info, Config>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
           VAULT , mint.key().as_ref(), 
        ],
        bump,
        constraint = vault.completed == false @ ErrorMessage::TradingEnd
    )]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK: only to check buyer can only buy token once
    #[account(
        init,
        space = 8,
        payer= buyer,
        seeds = [
            PDA_CHECK , buyer.key().as_ref(),mint.key().as_ref(), 
        ],
        bump
    )]
    pub pda_buyer: AccountInfo<'info>,
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
    pub associate_user: Account<'info, TokenAccount>,
    /// CHECK: receive fund
    #[account(mut)]
    pub fund: AccountInfo<'info>,
    /// CHECK:
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler_buy(
    ctx: Context<Buy>,
    max_amount_sol: u64,
) -> Result<(Pubkey, Pubkey, u64, u64, u64, u64, bool)> {
    let config_account = &ctx.accounts.config;
    let vault= &mut ctx.accounts.vault;
    let mut current_total_sol = vault.get_lamports()-1746960u64; // fee init create acc
    require!(max_amount_sol > 0 ,ErrorMessage::InvalidAmountSol);
    let (amount_sol, _fee_platform, fee_fund) = calculate_fee(
        max_amount_sol,
        0,
        config_account.fee_fund,
    )
    .unwrap();

    require!(amount_sol > 0, ErrorMessage::InvalidAmountSol);

    require!(amount_sol.checked_add(current_total_sol).unwrap().le(&vault.sol_target), ErrorMessage::InvalidAmountSolTrade);

    // transfer fee to fee_receiver
    // if fee_platform > 0 {
    //     transfer_native_to_account(
    //         ctx.accounts.buyer.to_account_info(),
    //         ctx.accounts.fee_receiver.to_account_info(),
    //         fee_platform,
    //         ctx.accounts.system_program.to_account_info(),
    //         None,
    //     )?;
    // }

    if fee_fund > 0 {
        transfer_native_to_account(
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.fund.to_account_info(),
            fee_fund,
            ctx.accounts.system_program.to_account_info(),
            None,
        )?;
    }

    transfer_native_to_account(
        ctx.accounts.buyer.to_account_info(),
        vault.to_account_info(),
        max_amount_sol,
        ctx.accounts.system_program.to_account_info(),
        None,
    )?;
    current_total_sol = current_total_sol.checked_add(max_amount_sol).unwrap();


    // calculate token receive
    let token_reserve = ctx.accounts.associate_vault.amount;
    let price = vault.price;


    let amount_token_out = (max_amount_sol as u128).checked_mul(1e6 as u128).unwrap().checked_div(price as u128).unwrap() as u64;
    let remaining_token = token_reserve.checked_sub(amount_token_out).unwrap();

    require!(
        amount_token_out > 0 && amount_token_out as u64 <= token_reserve,
        ErrorMessage::InvalidAmountToken
    );

    let amount_token_holding = ctx.accounts.associate_user.amount + amount_token_out;
   
    require!(
        amount_token_holding <= vault.max_token_buy || ctx.accounts.buyer.key() == vault.creator,
        ErrorMessage::ExceedMaximum
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

    Ok((
        ctx.accounts.mint.key(),
        ctx.accounts.buyer.key(),
        current_total_sol,
        remaining_token,
        max_amount_sol,
        amount_token_out as u64,
        vault.completed,
    ))
}
