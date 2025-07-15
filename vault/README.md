# 🔍 Vault Program — Test Case Logs

This document summarizes the test cases for the Anchor-based Vault program and their associated transaction signatures, balances, and program behavior.

---

## ✅ Test Case: `Initialize vault!`

- **Transaction Signature:**  
  `FZWnPPMB3v7s5737VgPCcUS8a4RnoehoMYh8aEfsmtmgDnYtrr1wzKZxjWKgrh2TeRF4W2t1ASMv7oA8JSqd5GB`

- **Vault Account:**  
  `lamports: 890880`  
  `owner: System Program (11111111111111111111111111111111)`

- **VaultState Account:**  
  `lamports: 960480`  
  `owner: Program ID EQVUdxnQSqFyAsQ1smUpbY47afHYb7x354BzDHaLnxd9`  
  `space: 10 bytes`

- ❗ **Error on rerun:**  
  `Allocate: account ... already in use` (expected — PDA already exists)

---

## ✅ Test Case: `Deposit 1 SOL!`

- **Transaction Signature:**  
  `3jC4UFYGrBYgW986jWL5Ak2uuo4bA5jAdtNAnSfrfoCV2kfEDbLtXmTBEsckRYwUP5ewbasCupgAAFAjYFHsUT5Y`

- **Vault Balance After Deposit:**  
  `1,000,890,880 lamports` (~1 SOL)

---

## ✅ Test Case: `Withdraw 0.5 SOL from vault`

- **Transaction Signature:**  
  `26S1hinkpbrzyQziUwtXXgL2mAvmPduCWzZt7Ru6eGdvNdVFQmDTkN8ncLthMj6cTiQBbtova8pni9xJB9qq4kj1`

- **Signer Balance Before:**  
  `7,381,148,815 lamports`  
  **Signer Balance After:**  
  `7,881,143,815 lamports`

- **Vault Balance Before:**  
  `1,500,890,880 lamports`  
  **Vault Balance After:**  
  `1,000,890,880 lamports`

---

## ✅ Test Case: `Closes the vault!`

- **Transaction Signature:**  
  `5SNaWB56qUiNi8H7iHZrRFbo7cWt8n1dYct4WMbvidsh4H2t7Xz9R9saGepPYyK48WVpr6y4ejk1nspYuPtEVJ5g`

- **Signer Balance Before:**  
  `7,881,143,815 lamports`  
  **Signer Balance After:**  
  `8,882,990,175 lamports`

- **Vault Balance Before:**  
  `1,000,890,880 lamports`  
  **Vault Balance After:**  
  `0 lamports` ✅

---
