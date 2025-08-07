use anchor_lang::prelude::*;

#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CityName {
    Solaria,
    Phantom,
    Jupiter,
    Marinade,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OpponentType {
    Trainer,
    GymLeader,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BattleStatus {
    Active,
    PlayerWon,
    PlayerLost,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NPCConfig {
    pub city: CityName,
    pub opponent_type: OpponentType,
    pub name: String,
    pub monsters: Vec<String>, 
    pub monster_levels: Vec<u8>, 
}