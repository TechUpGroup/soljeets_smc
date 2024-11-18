use anchor_lang::prelude::*;

#[account]
pub struct Holder {
    pub mint: Pubkey,
    pub holder: Pubkey,
    pub amount_token_received: u64
}

impl Holder {
    pub const LEN: usize = 8 + 1 * 8 + 2 * 32;
}
