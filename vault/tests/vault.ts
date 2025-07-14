import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const signer = provider.wallet.publicKey;

	const program = anchor.workspace.vault as Program<Vault>;

	const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("state"), provider.wallet.publicKey.toBuffer()],
		program.programId
	)[0];

	const vault = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("vault"), vaultState.toBuffer()],
		program.programId
	)[0];

	it("Is initialized!", async () => {
		const tx = await program.methods
			.initialize()
			.accountsPartial({
				signer,
				vaultState,
				vault,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();

		console.log("Transaction Signature(Initialize) : ", tx);
		console.log(
			"vault(Initialize) : ",
			await provider.connection.getAccountInfo(vault)
		);
		console.log(
			"vaultState : ",
			await provider.connection.getAccountInfo(vaultState)
		);
	});

	it("Deposit 1 SOL!", async () => {
		const tx = await program.methods
			.deposit(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
			.accountsPartial({
				signer,
				vaultState,
				vault,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();
		console.log("Transaction Signature(Initialize) : ", tx);
		console.log(
			"vault Balance: ",
			(await provider.connection.getBalance(vault)).toString()
		);
	});

	it("Withdraws 0.5 SOL from vault", async () => {
		const initialSignerBalance = await provider.connection.getBalance(
			provider.wallet.publicKey
		);

		const initialVaultBalance = await provider.connection.getBalance(vault);

		const tx = await program.methods
			.withdraw(new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL))
			.accountsPartial({
				signer,
				vaultState,
				vault,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.rpc();

		const finalSignerBalance = await provider.connection.getBalance(signer);

		const finalVaultBalance = await provider.connection.getBalance(vault);

		console.log("Withdraw Tx Signature : ", tx);
		console.log("Signer balance before : ", initialSignerBalance);
		console.log("Signer balance After: ", finalSignerBalance);
		console.log("Vault balance before : ", initialVaultBalance);
		console.log("Vault balance after : ", finalVaultBalance);
	});

	it("Closes the vault!", async () => {
		const initialSignerBalance = await provider.connection.getBalance(
			provider.wallet.publicKey
		);

		const initialVaultBalance = await provider.connection.getBalance(vault);

		const closeTx = await program.methods.close().accountsPartial({
			signer,
			vaultState,
			vault,
			systemProgram: anchor.web3.SystemProgram.programId,
		});

		const finalSignerBalance = await provider.connection.getBalance(signer);

		const finalVaultBalance = await provider.connection.getBalance(vault);

		console.log("Close Tx Signature : ", closeTx);
		console.log("Signer balance before : ", initialSignerBalance);
		console.log("Signer balance After: ", finalSignerBalance);
		console.log("Vault balance before : ", initialVaultBalance);
		console.log("Vault balance after : ", finalVaultBalance);
	});
});

// Deployed program :
// Program Id: EQVUdxnQSqFyAsQ1smUpbY47afHYb7x354BzDHaLnxd9

// Signature: 2qrHNVFR5tDpSWFbvzZJqhcAhEeVY7XnpSTDt5NS8Q7zWtpTbMCCUdNJYX8xhHFvwCP8UfgHe36XetosmRyDZWhh
