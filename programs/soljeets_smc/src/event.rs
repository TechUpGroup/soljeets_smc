use anchor_lang::prelude::*;

#[event]
pub struct TokenCreatedEvent {
    pub token: Pubkey,
    pub creator: Pubkey,
    pub total_supply: u64,
    pub target_jeets_score: u16
}

#[event]
pub struct TradingEvent {
    pub token: Pubkey,
    pub account: Pubkey,
    pub amount_sol: u64,
    pub amount_token: u64,
    pub is_buy: bool,
    pub virtual_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub completed: bool,
}

#[event]
pub struct Transfer {
    pub timestamp: u64,
    pub remain: Pubkey,
    pub transfer_amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub token: Pubkey,
    pub account: Pubkey,
    pub amount_sol: u64,
    pub amount_token: u64,
}
