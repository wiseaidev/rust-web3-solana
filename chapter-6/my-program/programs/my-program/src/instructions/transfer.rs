use crate::error::ErrorMsg;
use crate::state::Transfer;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let from_account = &mut ctx.accounts.from;
    let to_account = &mut ctx.accounts.to;

    if from_account.data < amount {
        return Err(ErrorMsg::InsufficientFunds.into());
    }

    from_account.data -= amount;
    to_account.data += amount;

    Ok(())
}
