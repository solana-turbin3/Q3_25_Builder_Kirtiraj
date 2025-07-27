import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";

describe("marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.marketplace as Program<Marketplace>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

// Program Id: 3Niwj9rzhrqBkVHt5an8Fdswtay5fcMG8T9cgujt5eQy

// Signature: 4z3Uay5T9impYwdUDVSQ3JbxidYKCLbeeM15C3BeoXgFWzVf86M3T1K6tevEfhaxBPE44fCy4uFR9d7drVB22zER