pub use anchor_lang::prelude::*;

pub fn calculate_token_out(
    invarient: u128,
    virtual_in_reserve: u128,
    virtual_out_reserve: u128,
    token_in_reserve: u128,
    token_out_reserve: u128,
    amount_in: u128,
) -> Result<(u128, u128, u128)> {
    let virtual_in_reserve_after = virtual_in_reserve.checked_add(amount_in).unwrap();
    let virtual_out_reserve_after = invarient.checked_div(virtual_in_reserve_after).unwrap();
    let amount_out = virtual_out_reserve
        .checked_sub(virtual_out_reserve_after)
        .unwrap();
    // let amount_out = virtual_out_reserve
    //     .checked_mul(amount_in)
    //     .unwrap()
    //     .checked_div(virtual_in_reserve)
    //     .unwrap();

    let token_in_reserve_after = token_in_reserve.checked_add(amount_in).unwrap();
    let token_out_reserve_after = token_out_reserve.checked_sub(amount_out).unwrap_or(0);

    Ok((token_out_reserve_after, token_in_reserve_after, amount_out))
}
