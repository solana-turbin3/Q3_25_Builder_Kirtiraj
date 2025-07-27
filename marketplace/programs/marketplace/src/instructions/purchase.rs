use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{close_account, mint_to, transfer_checked, CloseAccount, MintTo, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::MarketplaceError,
    state::{Listing, Marketplace},
};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = mint,
        associated_token::token_program = token_program
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), mint.key().as_ref()],
        bump = listing.bump,
        close = maker
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::authority = marketplace,
        mint::decimals = 6
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = rewards_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Purchase<'info> {
    pub fn execute_purchase(&mut self) -> Result<()> {
        // flow : taker pays for nft(excluding the platform fee)-> nft transfer from vault_ata to taker's ata -> reward  the taker for buying nft from this platform.

        self.pay()?;
        self.transfer_nft()?;
        self.reward_buyer()?;
        Ok(())
    }

    fn pay(&mut self) -> Result<()> {
        let fee = self.marketplace.fee as u64;

        let marketplace_fee = self
            .listing
            .price
            .checked_mul(fee)
            .and_then(|v| v.checked_div(100))
            .ok_or(MarketplaceError::NumericalOverflow)?;

        // transfer marketplace fee to the marketplace's treasury
        let cpi_fee_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.taker.to_account_info(),
                to: self.treasury.to_account_info(),
            },
        );
        transfer(cpi_fee_ctx, marketplace_fee)?;

        let maker_amount = self
            .listing
            .price
            .checked_sub(marketplace_fee)
            .ok_or(MarketplaceError::NumericalOverflow)?;

        // transfer the amount() from taker to maker
        let cpi_maker_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_maker_ctx =
            CpiContext::new(self.system_program.to_account_info(), cpi_maker_accounts);
        transfer(cpi_maker_ctx, maker_amount)?;

        Ok(())
    }

    fn transfer_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let seeds = &[
            b"listing",
            self.marketplace.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            to: self.taker_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, 1, 0)?;

        let cpi_close_accounts = CloseAccount {
            account: self.vault_ata.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_close_accounts,
            signer_seeds,
        );

        close_account(close_ctx)?;
        Ok(())
    }

    fn reward_buyer(&mut self) -> Result<()> {
        let seeds = &[
            b"marketplace",
            self.marketplace.name.as_bytes(),
            &[self.marketplace.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = MintTo {
            to: self.taker.to_account_info(),
            mint: self.rewards_mint.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        mint_to(cpi_ctx, 1)?;
        Ok(())
    }
}

// PAY :
// Calculate marketplace fee as a percentage of the NFT price.
// For example, if `price = 1 SOL` (i.e., 1_000_000_000 lamports) and `fee = 5`,
// then marketplace_fee = (1_000_000_000 * 5) / 100 = 50_000_000 lamports (0.05 SOL).

// maker_amount is the amount asked by the maker(seller) - platform fee

// transfer_nft :
// send nft from vault to the taker and close the listing

// rewards buyer :
// mint 1 token to the buyer as a reward for using this platform
