use anchor_lang::prelude::*;
use crate::error::CustomError;
use crate::state::*;

#[derive(Accounts)]
pub struct StartBattle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", signer.key().as_ref()],
        bump = player_account.bump
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [b"battle", signer.key().as_ref()],
        bump,
        space = BattleState::LEN
    )]
    pub battle_state: Account<'info, BattleState>,

    pub system_program: Program<'info, System>
}

impl<'info> StartBattle<'info> {
    pub fn start_battle(&mut self, npc_id: u8, bumps: &StartBattleBumps) -> Result<()> {
        require!(
            self.player_account.monster.current_hp > 0,
            CustomError::MonsterFainted
        );

        // hardcode for phase 1(TODO: can implement proper data lookups)
        let npc_monster = Monster {
            level: 1,
            current_hp: 30,
            max_hp: 30,
            current_xp: 0,
            max_xp: 100,
            moves: vec![MoveType::Tackle],
        };

        self.battle_state.set_inner(BattleState {
            player: self.player_account.key(),
            npc_id,
            player_monster: self.player_account.monster.clone(),
            npc_monster,
            current_turn: 0,
            status: BattleStatus::Active,
            bump: bumps.battle_state,
        });

        msg!("Battle started against NPC ID: {}", npc_id);

        Ok(())
    }
}