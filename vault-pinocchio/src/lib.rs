#![no_std]
#![allow(unexpected_cfgs)]

use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    program_error::ProgramError, 
    pubkey::Pubkey, 
    ProgramResult,
    nostd_panic_handler
};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod instructions;
pub use instructions::*;

pinocchio_pubkey::declare_id!("EqJb8hX4J7naqsqXQVhRNZbg3g5f7mCCtvC6Ro8RF8XY");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}