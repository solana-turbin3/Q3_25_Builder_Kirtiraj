#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8a9q6S7toSge9K6usiMugrm2AbyJ8pLB23infa5jT8AT");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(
            seed,
            fee,
            authority,
            ctx.bumps
        )?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()>{
        ctx.accounts.deposit(amount, max_x, max_y)?;

        Ok(())
    }
}
