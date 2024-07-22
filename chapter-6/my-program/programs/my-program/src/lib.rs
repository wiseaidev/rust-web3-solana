pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EPUMSZYPAKa11SvtGhD6izCNECRkS1gPNkAHsN7HnUkZ");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        transfer::handler(ctx, amount)
    }
}
