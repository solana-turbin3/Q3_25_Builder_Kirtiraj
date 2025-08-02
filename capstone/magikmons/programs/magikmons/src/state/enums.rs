use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MoveType {
    Tackle,
    Bite,
    Heal
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OpponentType { 
    Trainer,
    GymLeader
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BattleStatus {
    Active,
    PlayerWon,
    PlayerLost
}