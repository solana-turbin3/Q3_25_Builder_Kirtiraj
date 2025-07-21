#![warn(deprecated)]
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

    pub fn initialize(ctx: Context<Initialize>, config_id: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(
            config_id,
            fee,
            authority,
            &ctx.bumps
        )
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()>{
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, min_x: u64, min_y: u64, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(min_x, min_y, amount)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64, is_x: bool) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }

    pub fn lock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.lock()
    }

    pub fn unlock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.unlock()
    }
}
