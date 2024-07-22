use crate::state::Initialize;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let my_account = &mut ctx.accounts.my_account;
    my_account.authority = ctx.accounts.authority.key();

    Ok(())
}
