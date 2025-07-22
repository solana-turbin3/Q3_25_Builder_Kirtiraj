use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi,
            FreezeDelegatedAccountCpiAccounts
        },
        MasterEditionAccount,
        Metadata,
        MetadataAccount
    },
    token::{approve, Approve, Mint, Token, TokenAccount}
};

use crate::{error::NFTStakingError, state::*};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub nft_mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.user_account_bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.stake_config_bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        seeds = [b"stake", nft_mint.key().as_ref(), config.key().as_ref()],
        bump,
        space = 8 + StakeAccount::INIT_SPACE
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    // freeze nft
    // delegate nft
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amount_staked < self.config.max_stake, NFTStakingError::MaxStakeReachedError
        );

        // set the values in the stake account
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            nft_mint: self.nft_mint.key(),
            stake_at: Clock::get()?.unix_timestamp,
            stake_account_bump: bumps.stake_account
        });

        // delegate the nft_ata account 
        self.delegate_nft_account_authority()?;        

        // now since stake_account gets approval to use the user_nft_ata, it can freeze it for some time

        self.freeze_account()?;

        // increase the total stake amount in the user account
        self.user_account.amount_staked += 1;

        Ok(())
    }

    pub fn delegate_nft_account_authority(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.user_nft_ata.to_account_info(), // the account we are approving
            delegate: self.stake_account.to_account_info(), // Who are we approving
            authority: self.user.to_account_info() // Owner of the token account (must sign)
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // context and amount we are approving
        approve(cpi_ctx, 1)?;

        Ok(())
    }
    
    pub fn freeze_account(&mut self) -> Result<()> {
        let cpi_program = &self.metadata_program.to_account_info();

        let cpi_accounts = FreezeDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.user_nft_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_program: &self.token_program.to_account_info()
        };

        let seeds = &[
            b"stake",
            self.nft_mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.stake_account_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new(cpi_program, cpi_accounts).invoke_signed(signer_seeds)?;

        Ok(())
    }
}