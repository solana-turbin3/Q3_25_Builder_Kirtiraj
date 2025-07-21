import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";

describe("amm", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.amm as Program<Amm>;

	it("Is initialized!", async () => {
		// Add your test here.
		const tx = await program.methods.initialize().rpc();
		console.log("Your transaction signature", tx);
	});
});

// Program Id: 95PegswFoCfWCTxBZGxgHzNvAppiq2XyurqydcCDkMmf

// Signature: 3Doui3cuVXjhUScHWv53wZgmLHDDMKJf6zKoPQtL9T4Vy3nLa3wFUUE6W1dnek25TG4WJ6oN64iJ2qkQhcHPnb1X
