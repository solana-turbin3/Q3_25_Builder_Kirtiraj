# Magikmons Test Suite – Phase 2

This document summarizes the execution of the **Magikmons Phase 2** test suite.

---

## Deployment

* **Program ID**: `7drP7t7GV2zfuru8dnKwnj9TRUUZ6mA9pDfMgk6yTrpJ`
* **Deployment Signature**: `XgYzoRAvhTVB6JVCz2NsS4JZvGXa7kRHXcrRZ8McQtqqcosu1ZtLbUvP7KaPy2DYo2bnuhYWmGLNbeavY5xJPhF`

---

## Test Setup

* **Authority**: `BSgVj3bfNVoPNvWLGBn4LHaUwUoRSRS6H2NUP2GHXZKd`
* **Test Keypair**: `69AbNM6E9iSTxRx9ViSrKGzpMe9RA5oTJv87xjv87HHE`
* **Game Config**: `HQ4Gs8BpSLWA4WHap4eVRVLVMy4x6o7bZK1463HLmpY8`
* **Player Account**: `F4NDDkAspUuepFNZwFMcgJrwyPThqJkf5JRP9nAkBoW7`
* **Treasury**: `Hhw1npSDi4Aoq6RoBVD89w6MACPNRF5jhPfe6WVqfzYB`

---

## Key Transactions

### 1. Game Initialization

* **Signature**: `4dkU1RKWMRMGnfAUL9ufBhDWTypAmN4rBdQixJdMYCjz2WGpSqxiyT4etZwFEpWXrewfatJBYL64QuEc1SG6VmaG`
* Initialized with default NPCs:

  * Natty Node Nate
  * Liquidity Lord Andre
  * Devnet Whale Jeff

---

### 2. Player Creation

* **Signature**: `5NbxDQYDvGJfAuro5HWzDntEc4YZWxt24W9S1r1pP3oihXnTXkaKripZRpcPXYbd7b6fo17E4yWYoZb4pptsTkxp`
* Player `TestTrainer` created with a starter monster.

---

### 3. First Battle (vs Natty Node Nate)

* **Start Signature**: `Z2C9F86vWvMARgba2k8ESbmxNnJRg2ejYvAb9FQKTuYXvSav22yycMpizgnpi168RxEvn3uKRrfJmqbQifnNp8x`
* **End Signature**: `5iU4fdEQHjhkPBjP2WxdEUpKji9oWeyR3jJw7dcRfSPwkp7Lo8y8tUtX7xRi4m4UWuuuzgUE1h4SsiNhEUotnJVR`
* **Result**: Player won, gained 50 XP.

---

### 4. Travel System

* Travel to **Surf City** → `42RKPYEJ6M8G5S4zWb6J8Aw6kNVke2eMJSEFf5iLudRAtPbWpTfvRPyxBiNEQvHZ5yMnrzRcChjhNsxtCazfn4bd`
* Travel to **Sol City** → `29anPbqQ6fbTADS53p9dEQRaocXKbLMjYX3wBM8QNXRMwSguacnxMd2m2jS4k9HTVNaKWkx6NQtvJ91KML79a9D4`
* Travel back to **Turbine Town** → `25Hu5oUCyxCfTbkTFqw2RECap8ngUN5Dt7r1TE3nQa3PXVTobqgXeKAzWE5GZfhfJRdP4ijWgwWR6RNoaA3bny9h`

---

### 5. Second Battle (vs Liquidity Lord Andre)

* **Start Signature**: `5Yjyy58X1yhEaYWuP31mwpSL8z3y7MrVPdEtqhJgckAh88mkhbLuzS9sjxfVPeTMCHPpCXuBFfQ8FtJw7UPANxNq`
* **Result**: Player won. Monsters leveled up.

---

### 6. Gym Leader Battle (vs Devnet Whale Jeff)

* **Start Signature**: `2mhMkRqZW1aiy2S3QRxU6tQeDNMg5A3rYNP3PxW7Xg94NRrpe3Sybx9PLKYo6R2e9V8nbb7oNrdgRa55NfGwdkWB`
* **Result**: Player won. Earned **Turbine Badge**.

---

### 7. Add New NPC

* **Signature**: `2rps2nHkNtcyyvQ4rVrHeUDqc6dYqySGLvGWztEQCM6A3qMLYroUHNv92ZxEBZHhfD3u6Ui6NjxQt4P79Hz1773m`
* NPC Added: **Gasless Guru Shrinath** with monsters `w003`, `l002`.

---

## Final Player State

* **Player Name**: `TestTrainer`
* **Level**: 3
* **Badge(s)**: `[ turbine ]`
* **Battles Won**: 3 / 3
* **Monsters**:

  * `f001` → Level 3, 90/90 HP, Moves: `damage1, sleepyStatus, paralysisStatus`
* **Items**: `item_recoverHp x7`
* **Defeated NPCs**: Nate ✓, Andre ✓, Jeff ✓

---

✅ **All 15 test cases passed successfully.**
