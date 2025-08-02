use anchor_lang::prelude::*;
use crate::{BattleStatus, Monster};

#[account]
pub struct BattleState {
    pub player: Pubkey,
    pub npc_id: u8,
    pub player_monster: Monster,
    pub npc_monster: Monster,
    pub current_turn: u8,
    pub status: BattleStatus,
    pub bump: u8
}

impl BattleState { 
     pub const LEN: usize = 8 +             
        32 +            // player 
        1 +             // npc_id
        Monster::LEN * 2 + // player_monster + npc_monster
        1 +             // current_turn
        1 +             // status (u8)
        1;              // bump
}