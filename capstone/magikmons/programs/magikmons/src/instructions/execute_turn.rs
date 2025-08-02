use anchor_lang::prelude::*;
use crate::{error::CustomError, state::*};

#[derive(Accounts)]
pub struct ExecuteTurn<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"battle", signer.key().as_ref()],
        bump,
        constraint = battle_state.player == signer.key()
    )]
    pub battle_state: Account<'info, BattleState>
}

impl<'info> ExecuteTurn<'info> {
    pub fn execute_turn(&mut self, player_move: MoveType) -> Result<()> {
        let state: &mut BattleState = &mut self.battle_state;

        require!(state.status == BattleStatus::Active, CustomError::BattleEnded);

        {
            let npc = &mut state.npc_monster;
            let player = &mut state.player_monster;
            apply_move(player_move, npc, player);
        }

        if state.npc_monster.current_hp == 0 {
            state.status = BattleStatus::PlayerWon;
            msg!("Player WON!");
            return Ok(());
        }

        let npc_move = state.npc_monster.moves[0].clone();

        {
            let player = &mut state.player_monster;
            let npc = &mut state.npc_monster;
            apply_move(npc_move, player, npc);
        }

        if state.player_monster.current_hp == 0 {
            state.status = BattleStatus::PlayerLost;
            msg!("Player LOST!");
            return Ok(());
        }

        state.current_turn += 1;
        msg!("Turn {} executed", state.current_turn);

        Ok(())
    }
}

fn apply_move(mv: MoveType, target: &mut Monster, actor: &mut Monster) {
    match mv {
        MoveType::Tackle => {
            target.current_hp = target.current_hp.saturating_sub(20);
        }
        MoveType::Bite => {
            target.current_hp = target.current_hp.saturating_sub(30);
        }
        MoveType::Heal => {
            actor.current_hp = (actor.current_hp + 15).min(actor.max_hp);
        }
    }
}