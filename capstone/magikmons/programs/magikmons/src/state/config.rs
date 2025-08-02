use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameConfig {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub config_bump: u8,
}

impl GameConfig {
    pub const LEN: usize = 8 + 
        32 +    // authority
        32 +    // treasury
        1;    // config_bump
}