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

declare_id!("DXQ8SrqSJ35c2Z6rgssquV5zT2jBs4Qhe2s3aGXamgNe");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()>{
        ctx.accounts.stake(&ctx.bumps)
    }
    pub fn unstake(ctx: Context<Unstake>) -> Result<()>{
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()>{
        ctx.accounts.claim_rewards()
    }
}
