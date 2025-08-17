# Vault Pinocchio

A minimal Solana program written with **Pinocchio** (a lightweight framework alternative to Anchor).


## What is it?
The Vault allows users to:
- **Deposit** lamports into a vault PDA owned by the program.  
- **Withdraw** lamports back to wallet (only by the vault’s owner).

Each vault is uniquely derived from the user’s public key + a static seed, ensuring only the program can move funds in/out.
