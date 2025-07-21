use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        transfer_checked,
        Mint,
        Token,
        TokenAccount,
        TransferChecked,
        Transfer,
        transfer
    }
};
use constant_product_curve::{ ConstantProduct, LiquidityPair};

use crate::{error::AmmError, program::Amm, state::Config};

#[derive(Accounts)]
pub struct Swap<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.config_id.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>
}

impl<'info> Swap<'info> {
    // user swaps some amount of tokens(amount_in) and expects some amount in return(min_amount_out)
    pub fn swap(&mut self, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount_in > 0, AmmError::InvalidAmount);

        // initialize constant product curve based on the current pool
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,  // total x tokens in pool
            self.vault_y.amount,  // total y tokens in pool
            self.mint_lp.supply,  // current LP tokens supply
            self.config.fee,   // trading fees in basis points
            None   // precision
        ).map_err(AmmError::from)?;

        // determine the token swapped
        let pair = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };

        let swap_result = curve.swap(pair, amount_in, min_amount_out).map_err(AmmError::from)?;

        require!(swap_result.deposit != 0, AmmError::InvalidAmount);
        require!(swap_result.withdraw != 0, AmmError::InvalidAmount);
        

        // if user deposits mint_x token then he can withdraw mint_y token
        // if he deposits mint_y token then he can withdraw mint_x token
        
        self.deposit_token(is_x, swap_result.deposit)?;
        
        self.withdraw_tokens(is_x, swap_result.withdraw)?;

        Ok(())
    }

    pub fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to ) = match is_x {
            true => (self.user_x_ata.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y_ata.to_account_info(), self.vault_y.to_account_info())
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from,
            to,
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_y.to_account_info(), self.user_y_ata.to_account_info()),
            false => (self.vault_x.to_account_info(), self.user_x_ata.to_account_info())
        };

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info()
        };

        let seeds = [
            &b"config"[..],
            &self.config.config_id.to_le_bytes(),
            &[self.config.config_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}