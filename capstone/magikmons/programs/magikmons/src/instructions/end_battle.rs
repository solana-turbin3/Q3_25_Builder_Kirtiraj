use anchor_lang::prelude::*;

use crate::{state::*, error::CustomError};

#[derive(Accounts)]
pub struct EndBattle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(
        mut,
        seeds = [b"battle", signer.key().as_ref()],
        bump,
        constraint = battle_state.player == signer.key()
    )]
    pub battle_state: Account<'info, BattleState>
}

impl<'info> EndBattle<'info> {
    pub fn end_battle(&mut self) -> Result<()>{
        let player = &mut self.player_account;
        let battle = &self.battle_state;

        require!(battle.status != BattleStatus::Active, CustomError::BattleStillOngoing);

        if(battle.status == BattleStatus::PlayerLost) {
            msg!("You Lost, No XP awarded.");
            return Ok(())
        }

        let npc_id = battle.npc_id;
        require!(!player.defeated_npcs[npc_id as usize], CustomError::AlreadyDefeated);

        if(npc_id == 0 || npc_id == 1){
            Self::apply_xp(player, &battle.player_monster)?;
        }

        if(npc_id == 2) {
            msg!("GYM Leader Defeated - You can claim NFT!")
        }

        player.defeated_npcs[npc_id as usize] = true;

        Ok(())
    }

    fn apply_xp(player: &mut Account<'info, PlayerAccount>, monster: &Monster) -> Result<()> {
        let mut updated_monster = monster.clone();
        updated_monster.current_xp += 50;

        if(updated_monster.current_xp >= updated_monster.max_xp){
            updated_monster.level += 1;
            updated_monster.max_hp += 20;
            updated_monster.current_hp = updated_monster.max_hp;
            updated_monster.current_xp = 0;
            updated_monster.max_xp += 50;

            msg!("Monster leveled up to level {}", updated_monster.level);
        }

        player.monster = updated_monster;
        msg!("Awarded 50 XP!");
        Ok(())
    }
}