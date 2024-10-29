use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub token_supply: u64,
    pub decimal: u8,
    pub initial_token_reserve: u64,
    pub sol_target: u64,
    pub max_token_buy: u64,
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub creator_buy: bool,
    pub price: u64,
    pub completed: bool,
}

impl Vault {
    pub const LEN: usize = 8 + 2* 1 + 1 + 6 * 8 + 2 * 32;
}
