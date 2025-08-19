import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { assert } from "chai";
import { createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("anchor-amm", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program=anchor.workspace.anchorAmm as Program<Amm>;

	const connection=anchor.AnchorProvider.env().connection;

	const keypair=anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet));

	let mintX: anchor.web3.PublicKey;
	let mintY: anchor.web3.PublicKey;
	let userAtaX: anchor.web3.PublicKey;
	let userAtaY: anchor.web3.PublicKey;
	let vaultAtaX: anchor.web3.PublicKey;
	let vaultAtaY: anchor.web3.PublicKey;
	let mintLP: anchor.web3.PublicKey;
	let config: anchor.web3.PublicKey;
	let userAtaLP: anchor.web3.PublicKey;

	const systemProgram=anchor.web3.SystemProgram.programId;
	const tokenProgram=anchor.utils.token.TOKEN_PROGRAM_ID;
	const associatedTokenProgram=anchor.utils.token.ASSOCIATED_PROGRAM_ID;

	const fee=3000;
	const seed=new anchor.BN(1);

	before(async () => {
		mintX=await createMint(
			connection,
			keypair,
			keypair.publicKey,
			null,
			9
		);

		console.log("Mint X:", mintX.toBase58());

		mintY=await createMint(
			connection,
			keypair,
			keypair.publicKey,
			null,
			9
		);

		console.log("Mint Y:", mintY.toBase58());

		userAtaX=(await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mintX,
			keypair.publicKey
		)).address;

		console.log(`User Mint X ATA: ${userAtaX.toBase58()}`);

		const mintXTx=await mintTo(
			connection,
			keypair,
			mintX,
			userAtaX,
			keypair.publicKey,
			1000000*Math.pow(10, 9)
		);
		console.log(`Mint Tx X: ${mintXTx}`);

		userAtaY=(await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mintY,
			keypair.publicKey
		)).address;
		console.log(`User Mint Y ATA: ${userAtaY.toBase58()}`);

		const mintYTx=await mintTo(
			connection,
			keypair,
			mintY,
			userAtaY,
			keypair.publicKey,
			1000000*Math.pow(10, 9)
		);
		console.log(`Mint Tx Y: ${mintYTx}`);


		const seedBuffer=Buffer.alloc(8);
		seedBuffer.writeBigUInt64LE(BigInt(seed.toNumber()));

		[config]=anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from("config"), seedBuffer],
			program.programId
		);

		[mintLP]=anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from("lp"), config.toBuffer()],
			program.programId
		)

		userAtaLP=await getAssociatedTokenAddress(
			mintLP,
			keypair.publicKey
		)

		vaultAtaX=await getAssociatedTokenAddress(
			mintX,
			config,
			true
		);

		vaultAtaY=await getAssociatedTokenAddress(
			mintY,
			config,
			true
		);

	});

	it("Initialize!", async () => {

		const tx=await program.methods.initialize(seed, fee, keypair.publicKey)
			.accountsStrict({
				initializer: keypair.publicKey,
				mintX,
				mintY,
				mintLP,
				vaultX: vaultAtaX,
				vaultY: vaultAtaY,
				config,
				systemProgram,
				associatedTokenProgram,
				tokenProgram,
			}).signers([keypair]).rpc();

		console.log("Initialize tx", tx);

		const configAccount=await program.account.config.fetch(config);
		assert(configAccount, "Config account not found");
		assert(configAccount.authority.equals(keypair.publicKey), "Initializer mismatch");
		assert.strictEqual(configAccount.fee, fee, "Fee mismatch");

		const vaultXInfo=await connection.getTokenAccountBalance(vaultAtaX);
		const vaultYInfo=await connection.getTokenAccountBalance(vaultAtaY);
		assert.strictEqual(vaultXInfo.value.uiAmount, 0, "Vault X is not empty");
		assert.strictEqual(vaultYInfo.value.uiAmount, 0, "Vault Y is not empty");
		const lpSupply=await connection.getTokenSupply(mintLP);
		assert.strictEqual(lpSupply.value.uiAmount, 0, "LP mint supply is not zero");
	});

	it("Deposit", async () => {
		const amount=new anchor.BN(40000*Math.pow(10, 6));
		const max_x=new anchor.BN(30000*Math.pow(10, 9));
		const max_y=new anchor.BN(50000*Math.pow(10, 9));

		const tx=await program.methods.deposit(amount, max_x, max_y)
			.accountsStrict({
				user: keypair.publicKey,
				mintX,
				mintY,
				mintLP,
				vaultX: vaultAtaX,
				vaultY: vaultAtaY,
				userX: userAtaX,
				userY: userAtaY,
				userLP: userAtaLP,
				config,
				systemProgram,
				associatedTokenProgram,
				tokenProgram,
			}).rpc();

		await confirmTx(tx);
		console.log("Deposit Tx: ", tx);

	});

	it("Performs Swap!", async () => {
		const amount=new anchor.BN(5000*Math.pow(10, 9));
		const min=new anchor.BN(100*Math.pow(10, 9));

		const is_x=false;

		const tx=await program.methods.swap(amount, min, is_x)
			.accountsStrict({
				user: keypair.publicKey,
				mintX,
				mintY,
				vaultX: vaultAtaX,
				vaultY: vaultAtaY,
				userXAta: userAtaX,
				userYAta: userAtaY,
				config,
				systemProgram,
				associatedTokenProgram,
				tokenProgram,
			}).rpc();

		await confirmTx(tx);
		console.log("Swap Tx: ", tx);
	})

	it("Performs withdraw!", async () => {
		const amount=new anchor.BN(30000*Math.pow(10, 6));
		const min_x=new anchor.BN(20000*Math.pow(10, 9));
		const min_y=new anchor.BN(40000*Math.pow(10, 9));

		const tx=await program.methods.withdraw(min_x, min_y, amount)
			.accountsStrict({
				withdrawer: keypair.publicKey,
				mintX,
				mintY,
				mintLP,
				vaultX: vaultAtaX,
				vaultY: vaultAtaY,
				withdrawerXAta: userAtaX,
				withdrawerYAta: userAtaY,
				withdrawerLpAta: userAtaLP,
				config,
				systemProgram,
				associatedTokenProgram,
				tokenProgram,
			}).rpc();

		await confirmTx(tx);
		console.log("Withdraw tx: ", tx);

	});

	it("performs lock!", async () => {

		const tx=await program.methods.lock()
			.accountsStrict({
				user: keypair.publicKey,
				config,
				systemProgram
			}).rpc();

		await confirmTx(tx);
		console.log("Lock Tx:", tx);

	});

	it("Performs Unlock!", async () => {

		const tx=await program.methods.lock()
			.accountsStrict({
				user: keypair.publicKey,
				config,
				systemProgram
			}).rpc();

		await confirmTx(tx);
		console.log("Unlock Tx: ", tx);

	});

	const confirmTx=async (signature: string) => {
		const latestBlockhash=await connection.getLatestBlockhash();
		await connection.confirmTransaction(
			{
				signature,
				...latestBlockhash,
			},
			"confirmed"
		)
	}
});
// Program Id: 95PegswFoCfWCTxBZGxgHzNvAppiq2XyurqydcCDkMmf

// Signature: 3Doui3cuVXjhUScHWv53wZgmLHDDMKJf6zKoPQtL9T4Vy3nLa3wFUUE6W1dnek25TG4WJ6oN64iJ2qkQhcHPnb1X
