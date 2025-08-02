use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Battle has already ended")]
    BattleEnded,
    
    #[msg("Invalid NPC ID")]
    InvalidNpcId,
    
    #[msg("Already defeated")]
    AlreadyDefeated,
    
    #[msg("Name too long")]
    NameTooLong,
    
    #[msg("Battle is still ongoing")]
    BattleStillOngoing,

    #[msg("Monster fainted!")]
    MonsterFainted
}
