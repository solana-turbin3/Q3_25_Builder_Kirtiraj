use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing{
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8
}

// maker is the person who creates the listing(wants to sell NFT)
// we also have the mint and price of the NFT(price is what the maker expects)