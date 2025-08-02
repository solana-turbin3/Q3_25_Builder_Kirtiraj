pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DbXeBiA4WT23AkVdVotTGVNccXXPBXXE8RM2GdpRdkP4");

#[program]
pub mod magikmons {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>,name: String) -> Result<()> {
        ctx.accounts.initialize_player(name)
    }

}
