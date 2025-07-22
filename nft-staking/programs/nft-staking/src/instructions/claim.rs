use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, mint_to, MintTo, Token, TokenAccount},
    associated_token::AssociatedToken
};

use crate::state::*;

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"seeds", config.key().as_ref()],
        bump
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"config"],
        bump = config.stake_config_bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.user_account_bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user
    )]
    pub user_reward_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Claim<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seeds = &[
            &b"config"[..],
            &[self.config.stake_config_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds
        );


        let _ = mint_to(cpi_ctx, self.user_account.points as u64 * 10_u64.pow(self.reward_mint.decimals as u32));

        self.user_account.points = 0;

        Ok(())
    }
}