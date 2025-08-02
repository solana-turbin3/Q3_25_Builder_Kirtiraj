use anchor_lang::prelude::*;

use crate::state::enums::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Monster{
    pub level: u8,
    pub current_hp: u32,
    pub max_hp: u32,
    pub current_xp: u32,
    pub max_xp: u32,
    pub moves: Vec<MoveType>
}

impl Monster {
    pub const MAX_MOVES: usize = 4;

    pub const LEN: usize = 1 + // level
    4 + 4 +     // current and max hp
    4 + 4 +     // current and max xp
    4 + Self::MAX_MOVES;    // vec(4) + max moves size

    // in order to create the initial monster(TODO: will try to make it dynamic/ add some other logic)
    pub fn starter() -> Self {
        Self {
            level: 1,
            current_hp: 50,
            max_hp: 50,
            current_xp: 0,
            max_xp: 100,
            moves: vec![MoveType::Tackle]
        }
    }
}