use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub token_supply: u64,
    pub decimal: u8,
    pub initial_token_reserve: u64,
    pub sol_target: u128,
    pub max_token_buy: u64,
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub creator_buy: bool,
    pub amount_token_received_per_slot: u64,
    pub price: u64,
    pub completed: bool,
}

impl Vault {
    pub const LEN: usize = 8 + 2 * 1 + 1 + 5 * 8 + 16 + 2 * 32;
}
