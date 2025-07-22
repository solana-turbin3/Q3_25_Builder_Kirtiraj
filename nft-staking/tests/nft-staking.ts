import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.nftStaking as Program<NftStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});


// Program Id: F85i1GmDX8ub2y1VZbtULu5SaLjcMdbfA8mqHvbZDnvB

// Signature: 3wh7Xenb6VhQpySJQDS7Hy5qXA7pkMxXjBvuMyzaH24FJqPzz7bSzdR6WHJBtzGc7gyXffY3Jj2iW3rdqmEAyako