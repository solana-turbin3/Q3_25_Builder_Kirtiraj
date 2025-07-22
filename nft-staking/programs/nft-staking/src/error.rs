use anchor_lang::prelude::*;

#[error_code]
pub enum NFTStakingError {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Max stake amount reached")]
    MaxStakeReachedError,

    #[msg("Freeze period has not elasped")]
    FreezePeriodNotElaspedError,

    #[msg("Insufficient staked NFT found")]
    InsufficientStake
}