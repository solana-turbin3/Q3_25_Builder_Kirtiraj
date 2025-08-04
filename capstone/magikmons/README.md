# Turbin3 Capstone(Magikmons)

On-chain RPG game built on Solana using Anchor. Battle against NPCs, level up your Magikmons, and progress through multiple stages of gameplay.

## Phase 1: Core Game Mechanics

### 🧩 Program Features

- [x] Initialize player account with name and base stats.
- [x] Start battle against predefined NPCs (by ID).
- [x] Simulate battle turns using `execute_turn` with attack/move options.
- [x] Handle XP gain and level-up logic after battle.
- [x] Prevent duplicate battle attempts once ended.
- [x] Restrict turn execution to ongoing battles only.
- [x] Award win status on defeating NPCs.
- [x] Basic testing for end-to-end game flow using `ts-mocha`.

### 🧪 Test Cases (TypeScript)

- [x] Player initialization success/failure
- [x] Turn execution and win condition
- [x] Prevent replaying finished battles
- [x] Invalid NPC battle prevention
- [x] Display final game state

---
