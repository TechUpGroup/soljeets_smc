use crate::PERCENT;
pub use anchor_lang::prelude::*;

pub fn calculate_fee(max_amount: u64, fee_platform: u16, fee_fund: u16) -> Result<(u64, u64, u64)> {
    let amount_fee_platform = max_amount
        .checked_mul(fee_platform as u64)
        .unwrap()
        .checked_div(PERCENT)
        .unwrap();
    let amount_fee_fund = max_amount
        .checked_mul(fee_fund as u64)
        .unwrap()
        .checked_div(PERCENT)
        .unwrap();

    let amount_sol = max_amount
    .checked_sub(amount_fee_platform)
    .unwrap()
    .checked_sub(amount_fee_fund)
    .unwrap();
    Ok((amount_sol, amount_fee_platform, amount_fee_fund))
}
