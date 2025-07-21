use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
    pub config_id: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub lp_bump: u8,
    pub config_bump: u8
}

// there can be multiple amm pools(based on x and y tokens), so each pool will have a config_id
// authority is optional(can be used to freeze/unfreeze pools)
// mint_x and mint_y are the mints of the two tokens present in the amm
// fee will be distributed to the liquidity providers
// lp_bump will be used to derive the PDA/mint address of the LP token(LP tokens will be distributed to the Liquidity providers and will later be burnt when they withdraw the liquidity that they provided)
// config_bump will be used to derive the PDA of the amm pool