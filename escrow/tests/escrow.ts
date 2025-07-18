import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import * as SPL from "@solana/spl-token";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import rawKeypair from "../keypair.json";

describe("escrow", () => {
	const keypair = Keypair.fromSecretKey(Uint8Array.from(rawKeypair));
	const provider = new anchor.AnchorProvider(
		new anchor.web3.Connection("<YOUR_API_KEY>", "confirmed"),
		new anchor.Wallet(keypair),
		{}
	);
	anchor.setProvider(provider);
	const program = anchor.workspace.escrow as Program<Escrow>;
	const programId = program.programId;
	const connection = provider.connection;
	const tokenProgram = SPL.TOKEN_PROGRAM_ID;

	// maker account
	const alice = anchor.web3.Keypair.generate();
	// taker account
	const bob = anchor.web3.Keypair.generate();

	// seed for creating escrow
	const EscrowSeed = new anchor.BN(19);

	// mint(maker -> taker)
	let mintA: PublicKey;
	// mint(taker -> maker)
	let mintB: PublicKey;

	// associated token account for maker(Alice) which will store mintA tokens
	let makerAtaA: SPL.Account;

	// associated token account for maker where mintB tokens will be received
	let makerAtaB: SPL.Account;
	// associated token account for taker(Bob) which will store mintB tokens
	let takerAtaB: SPL.Account;
	// associated token account for taker where mintA tokens will be received
	let takerAtaA: SPL.Account;

	let escrowPda: anchor.web3.PublicKey;
	let bump: number;
	let vaultAta: anchor.web3.PublicKey;

	// amount of tokens maker wants from taker
	const receive = new anchor.BN(2_000_000);
	// amount of tokens maker gives to taker(via vault)
	const deposit = new anchor.BN(1_000_000);

	const airdrop = async (
		connection: anchor.web3.Connection,
		publicKey: PublicKey
	) => {
		const txHash = await connection.requestAirdrop(
			publicKey,
			2 * LAMPORTS_PER_SOL
		);

		console.log(`Airdrop Successful: ${txHash}`);

		const confirm = await connection.confirmTransaction(
			txHash,
			"confirmed"
		);

		const balance = await connection.getBalance(publicKey);
		console.log(`Signature : ${JSON.stringify(confirm, null, 2)}`);
		return confirm;
	};

	// send SOL from your account to alice and bob(If airdrop's limit has been reached)
	const sendSol = async (
		provider: anchor.AnchorProvider,
		receiver: PublicKey,
		amountInSol: number
	) => {
		const tx = new Transaction().add(
			SystemProgram.transfer({
				fromPubkey: provider.wallet.publicKey,
				toPubkey: receiver,
				lamports: amountInSol * LAMPORTS_PER_SOL,
			})
		);

		const sig = await provider.sendAndConfirm(tx);
		console.log(`Sent ${amountInSol} SOL to ${receiver.toBase58()}`);
		console.log(`Signature: ${sig}`);
	};

	before(async () => {
		// await airdrop(connection, alice.publicKey);
		// await airdrop(connection, bob.publicKey);

		// send sol from your wallet instead of airdrop
		await sendSol(provider, alice.publicKey, 0.5);
		await sendSol(provider, bob.publicKey, 0.5);

		const aliceBalance = await connection.getBalance(alice.publicKey);
		const bobBalance = await connection.getBalance(bob.publicKey);
		console.log(`Alice Balance : ${aliceBalance}`);
		console.log(`Bob Balance : ${bobBalance}`);
		console.log(`Alice key : ${alice.publicKey}`);
		console.log(`Bob key : ${bob.publicKey}`);

		mintA = await SPL.createMint(
			provider.connection,
			alice,
			alice.publicKey,
			null,
			6,
			undefined,
			undefined,
			tokenProgram
		);
		mintB = await SPL.createMint(
			provider.connection,
			bob,
			bob.publicKey,
			null,
			6,
			undefined,
			undefined,
			tokenProgram
		);

		console.log("Mint A : ", mintA);
		console.log("Mint B : ", mintB);

		// alice's ata for token A
		makerAtaA = await SPL.getOrCreateAssociatedTokenAccount(
			connection,
			alice,
			mintA,
			alice.publicKey
		);

		makerAtaB = await SPL.getOrCreateAssociatedTokenAccount(
			connection,
			alice,
			mintB,
			alice.publicKey
		);

		// Bob's ata for token B
		takerAtaB = await SPL.getOrCreateAssociatedTokenAccount(
			connection,
			bob,
			mintB,
			bob.publicKey
		);

		takerAtaA = await SPL.getOrCreateAssociatedTokenAccount(
			connection,
			bob,
			mintA,
			bob.publicKey
		);

		console.log(`MakerAta A: ${makerAtaA}`);
		console.log(`MakerAta B: ${makerAtaB}`);
		console.log(`TakerAta B: ${takerAtaB}`);
		console.log(`TakerAta A: ${takerAtaA}`);

		let mintToAlice = await SPL.mintTo(
			provider.connection,
			alice,
			mintA,
			makerAtaA.address,
			alice,
			2_000_000,
			undefined,
			undefined,
			tokenProgram
		);
		console.log(`Token minted to makerAtaA: ${mintToAlice}`);

		let mintToBob = await SPL.mintTo(
			provider.connection,
			bob,
			mintB,
			takerAtaB.address,
			bob,
			2_000_000,
			undefined,
			undefined,
			tokenProgram
		);
		console.log(`Token minted to takerAtaB: ${mintToBob}`);

		[escrowPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from("escrow"),
				alice.publicKey.toBuffer(),
				EscrowSeed.toArrayLike(Buffer, "le", 8),
			],
			programId
		);

		console.log(
			`Escrow Account created successfully with bump : ${escrowPda} , ${bump}`
		);

		vaultAta = await SPL.getAssociatedTokenAddressSync(
			mintA,
			escrowPda, // owner of vault would be the escrow pda
			true, // true because vault is PDA so we allow it to be offCurve
			tokenProgram
		);
		console.log(`Vault Address : ${vaultAta}`);
	});

	it("Should create escrow and deposit tokens", async () => {
		const tx = await program.methods
			.make(EscrowSeed, receive, deposit)
			.accountsPartial({
				maker: alice.publicKey,
				mintA,
				mintB,
				makerAtaA: makerAtaA.address,
				escrow: escrowPda,
				vault: vaultAta,
				tokenProgram: SPL.TOKEN_PROGRAM_ID,
				associatedTokenProgram: SPL.ASSOCIATED_TOKEN_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
			})
			.signers([alice])
			.rpc();

		console.log(`Escrow created successfully! tx : ${tx}`);
	});

	it("Taker should take the deal", async () => {
		const tx = await program.methods
			.take()
			.accountsPartial({
				escrow: escrowPda,
				taker: bob.publicKey,
				maker: alice.publicKey,
				mintA,
				mintB,
				takerAtaB: takerAtaB.address,
				vault: vaultAta,
				makerAtaB: makerAtaB.address,
				takerAtaA: takerAtaA.address,
				tokenProgram: SPL.TOKEN_PROGRAM_ID,
				associatedTokenProgram: SPL.ASSOCIATED_TOKEN_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
			})
			.signers([bob])
			.rpc();

		console.log(`Bob(Taker) accepts the deal(Takes the token): ${tx}`);
	});

	it("Create Escrow again(for testing refund)", async () => {
		const tx = await program.methods
			.make(EscrowSeed, receive, deposit)
			.accountsPartial({
				maker: alice.publicKey,
				mintA,
				mintB,
				makerAtaA: makerAtaA.address,
				escrow: escrowPda,
				vault: vaultAta,
				tokenProgram: SPL.TOKEN_PROGRAM_ID,
				associatedTokenProgram: SPL.ASSOCIATED_TOKEN_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
			})
			.signers([alice])
			.rpc();

		console.log("recreated escrow: ", tx);
	});

	it("Refunds tokenA to maker(Alice)", async () => {
		const tx = await program.methods
			.refund()
			.accountsPartial({
				escrow: escrowPda,
				maker: alice.publicKey,
				mintA,
				makerAtaA: makerAtaA.address,
				vault: vaultAta,
				associatedTokenProgram: SPL.ASSOCIATED_TOKEN_PROGRAM_ID,
				tokenProgram: SPL.TOKEN_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
			})
			.signers([alice])
			.rpc();

		console.log(`Refund successful! : ${tx}`);
	});
});

// Program Id: H7tt42VxRDfuPpc7VecKeRgHRpXBSsyc8r3cBVxjUHev

// Signature: CPEdjjiojHEatTmGQGgmHiFTdA7ZwcGJ1ch4FQQiALSUaD4nBrZHo3np947ffFDdJRZoTkLHvhDmWqckwBY6SoW

// TakerAtaA = 445ozbQzVLsZsjfvmZZ2qXURprzXXJqbGCPuxLsE1BuB
// TakerAtaB = 42e6Yfuxg175CPpCXcN1LA3WUB1sYSZWLe9j8sZdgsHT
// MakerAtaA = CGKHkQsXwPntZrLPpZKyj3k7dWGUp9WCYqmqhMhYaufJ
// MakerAtaB = 7adC92BS141KoUWAeptDGZbsgNE6DjvwQvaa1E8VNx1K
