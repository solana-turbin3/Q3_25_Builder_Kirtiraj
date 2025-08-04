pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ge6s5GhQV5iG96aX2vWDtSEnvh1oRjC9T5JKaNZVsqQp");

#[program]
pub mod magikmons {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>, name: String) -> Result<()> {
        ctx.accounts.initialize_player(name, &ctx.bumps)
    }

    pub fn start_battle(ctx: Context<StartBattle>, npc_id: u8) -> Result<()> {
        ctx.accounts.start_battle(npc_id, &ctx.bumps)
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>, player_move: MoveType) -> Result<()> {
        ctx.accounts.execute_turn(player_move)
    }


    pub fn end_battle(ctx: Context<EndBattle>) -> Result<()> {
        ctx.accounts.end_battle()
    }
}
