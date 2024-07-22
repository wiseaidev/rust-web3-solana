use crate::state::Initialize;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Initialize>, amount: u64) -> Result<()> {
    let my_account = &mut ctx.accounts.my_account;
    my_account.authority = ctx.accounts.authority.key();

    my_account.data = amount;

    Ok(())
}
