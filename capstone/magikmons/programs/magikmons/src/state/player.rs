use anchor_lang::prelude::*;
use crate::state::monster::Monster;

#[account]
pub struct PlayerAccount {
    pub owner: Pubkey,
    pub name: String,
    pub current_city: u8,
    pub defeated_npcs: Vec<bool>,
    pub monster: Monster,
    pub bump: u8
}

impl PlayerAccount {
    pub const MAX_NAME_LEN: usize = 32;
    pub const MAX_NPCS: usize = 3;
    pub const MAX_MOVES: usize = 4;

    pub const LEN: usize = 8 + 
    32 +  // owner's pubkey 
    4 + Self::MAX_NAME_LEN +    // string len(4) + total chars
    1 + // current city
    4 + Self::MAX_NPCS +    // defeated_npcs(4 for the vec
    Monster::LEN + 
    1;  // bump
}