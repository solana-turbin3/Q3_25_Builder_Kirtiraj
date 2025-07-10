#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
pub mod state;
pub use state::*;

declare_id!("8NCwiCxMp5w41EjWiCbkBxKrKEPEoiw64J4U9s5kfnvm");

#[program]
pub mod escrow {
    use super::*;
}

