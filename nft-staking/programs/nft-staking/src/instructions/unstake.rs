use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi,
            ThawDelegatedAccountCpiAccounts
        }, MasterEditionAccount, Metadata, MetadataAccount
    }, token::{Mint, Token, TokenAccount, revoke, Revoke}
};

use crate::{error::NFTStakingError, state::*};

#[derive(Accounts)]
pub struct Unstake<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.user_account_bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.stake_config_bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake", nft_mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> Unstake<'info> {
    // flow => unfreeze nft -> undelegate it -> reduce the amount staked from the user account
    pub fn unstake(&mut self) -> Result<()>{
        require!(
            self.user_account.amount_staked >= 1, 
            NFTStakingError::InsufficientStake
        );

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.stake_at) / 86400) as u32;

        require!(time_elapsed > self.config.freeze_period, NFTStakingError::FreezePeriodNotElaspedError);

        self.user_account.points += (
            self.config.points_per_stake as u32
        ) * time_elapsed;

        self.unfreeze_nft()?;
        self.undelegate_nft_authority()?;

        self.user_account.amount_staked -= 1;

        Ok(())
    }

    pub fn unfreeze_nft(&mut self) -> Result<()> {
        let cpi_program = &self.metadata_program.to_account_info();

        let cpi_accounts = ThawDelegatedAccountCpiAccounts { 
            delegate: &self.stake_account.to_account_info(), 
            token_account: &mut self.user_nft_ata.to_account_info(), 
            edition: &self.edition.to_account_info(), 
            mint: &self.nft_mint.to_account_info(), 
            token_program: &self.token_program.to_account_info() 
        };

        let seeds = [
            b"stake",
            self.nft_mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.stake_account_bump]
        ];

        let signer_seeds = &[
            &seeds[..]
        ];
        ThawDelegatedAccountCpi::new(cpi_program, cpi_accounts).invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn undelegate_nft_authority(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Revoke{
            source: self.user_nft_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_ctx)?;
        Ok(())
    }
}