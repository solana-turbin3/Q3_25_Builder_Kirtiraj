use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::error::CustomError;

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = PlayerAccount::LEN,
        seeds = [b"player", signer.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> InitializePlayer<'info> {
    pub fn initialize_player(&mut self, name:String, bumps: &InitializePlayerBumps) -> Result<()> {
        require!(name.len() <= PlayerAccount::MAX_NAME_LEN, CustomError::NameTooLong);

        let rent_fee = 10_000_000;

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, rent_fee)?;
    
        self.player_account.set_inner(PlayerAccount {
            owner: self.signer.key(),
            name,
            current_city: 0,
            defeated_npcs: vec![false, false, false],
            monster: Monster::starter(),
            bump: bumps.player_account
        });


        msg!("Player account initialized");

        Ok(())
    }
}