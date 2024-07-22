use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
/// The main account structure for the program, holding authority and data.
pub struct MyAccount {
    /// The public key of the authority (owner) of the account.
    pub authority: Pubkey,
    /// The data associated with the account.
    pub data: u64,
}

#[derive(Accounts)]
#[instruction()]
/// Context for initializing a new MyAccount.
pub struct Initialize<'info> {
    /// The account of the user initializing the MyAccount.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The MyAccount to be initialized.
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<MyAccount>(),
        seeds = [b"my_account", authority.key().as_ref()],
        bump
    )]
    pub my_account: Box<Account<'info, MyAccount>>,

    /// The system program, which is required for account operations.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
/// Context for transferring data between two MyAccounts.
pub struct Transfer<'info> {
    /// The MyAccount to transfer data from.
    #[account(
        mut,
        has_one = authority,
    )]
    pub from: Box<Account<'info, MyAccount>>,

    /// The MyAccount to transfer data to.
    #[account(
        mut,
        has_one = authority,
    )]
    pub to: Box<Account<'info, MyAccount>>,

    /// The account of the user performing the transfer.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The system program, which is required for account operations.
    pub system_program: Program<'info, System>,
}
