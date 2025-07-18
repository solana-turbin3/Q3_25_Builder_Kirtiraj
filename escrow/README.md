# Escrow Program — Workflow Overview & Transaction Logs

This Anchor-based Solana program facilitates a basic **token-for-token escrow** mechanism between two parties: **maker (Alice)** and **taker (Bob)**.

---

## 🔄 Workflow Overview

1. **Fund Accounts**  
   Fund Alice and Bob with SOL to cover transaction fees.

2. **Mint & Distribute Tokens**  
   - A token (`mintA`) is created and minted to Alice.
   - B token (`mintB`) is created and minted to Bob.

3. **Create Escrow (Alice as Maker)**  
   Alice escrows `mintA` into a vault account (PDA).

4. **Take Escrow (Bob as Taker)**  
   Bob transfers `mintB` to Alice and receives the escrowed `mintA`.

5. **Refund Escrow (Fallback Flow)**  
   If no taker arrives, Alice can refund the escrowed tokens.

---

## 📜 Execution & Logs

### ✅ Program Deployment
```

Program Id: 8UFewGTxzrSpojH9UDqbhVT8RMvK8p9Du6Rj8Ajdj2V9
Deploy success

```

---


---

### 🔹 1. Funding Alice & Bob
```

Sent 0.5 SOL to D8tKtFxbpRTkQfECLBBCtE6ZiJWf8W96bEFx8zRZErgJ
Signature: 4egjVctybwm1cLZwd8QFjLHAoYB7etobrmL3PTjAmSLh4DspPNXaUR75JwAPvLtAgHdJ6bnFDbq2eDomyPzasQbV

Sent 0.5 SOL to D7xmBQdLigDQjyWQ2DPrjbXEXrsTY1NAEaHzhFmj17De
Signature: 3qT9EYvC6h5X25Sn5jK2deC1wtthXYZXcAMsYXgyptgWEEvAN2ewWmufUqUFFKqFFAfQ6DWig6UwzzKmdMZZ92Cf

Alice Balance : 500000000
Bob Balance   : 500000000

```

---

### 🔹 2. Token Minting (A & B)
```

Mint A : AJ9jtNrcekVoWKP9PbTk4kMnqTbTnd7xdnguWKQLcPcv
Mint B : AJ9jtNrcekVoWKP9PbTk4kMnqTbTnd7xdnguWKQLcPcv  // same as mint A for test

Token minted to makerAtaA: 26zodhuFER4ggsdzRq7zLHwZiVaRwbLEu6x5JEDcVdwe8hL76UFtr8CtbRJhC6kvkuSY3WD1N2cszoLMyv9WPfPv
Token minted to takerAtaB: kzhHdr4Lcq9MNDTMiNWSDsd9XEv683GfYmeXjfpUd8Ra9ZVB5GWQp6XEPmWAtnju8mzrgYdV3cbBcKEeWMBn1hL

```

---

### 🔹 3. Escrow Creation (Maker = Alice)
```

Escrow Account created successfully with bump :
CkSFNnNgKzC1eHcrhi97nmp5sHyBDLAWqE2cmNYu6Roo , 255

Vault Address : FAUWLrv6ED82VqM5TDpUjSErgEJc9xRwSUn3mop9hhw8

Escrow created successfully!
Tx: 3nKaKbvKai5H6HvSerbeQbJEfDeHFJhMLVSGhHVSXZZAa2HnRAxKuXhk8MDSWrCBeVgfeDXTpS1Lykzd5sbvYwRK

```

---

### 🔹 4. Taker Accepts Deal (Bob)
```

Bob(Taker) accepts the deal
Tx: 67NvupH2xHZXoo3WUbSzAk7eDZRwi2sf2qySdAf7UrurNP6iKVqe7EjTu5TEh2FXM4rQJKFBVFDTFoc8c1BBzRLE

```

---

### 🔹 5. Refund Flow
```

Re-created escrow for refund test
Tx: 2h8YvyeVk2e62D3GwoTUnrMHp9QTybhsCJcwPRFXRQiM6bcrRQUT5QJqcR27rZ19WrFHvis33rv9XM3jy9ZfVZbJ

Refund successful!
Tx: 5Vxz5ickHhoNYNvvLCMDziyb6YSmTYF84cdYBRHYhA2PmZzmVeFKepfutfoixssHZaKLLa9ZmBVhBsTVUPbakQSa

```

---

## ✅ Test Summary
```

✔ Should create escrow and deposit tokens (2172ms)
✔ Taker should take the deal (1989ms)
✔ Create Escrow again (for testing refund) (2121ms)
✔ Refunds tokenA to maker (Alice) (2598ms)

4 passing (39s)

```
