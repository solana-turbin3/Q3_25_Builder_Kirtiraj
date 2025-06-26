import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const MPL_CORE_PROGRAM_ID = new PublicKey(
	"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);
const mintCollection = new PublicKey(
	"5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"
);

const mintTs = Keypair.generate();
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, new Wallet(keypair), {
	commitment: "confirmed",
});
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(
	account_seeds,
	program.programId
);
const [authorityPda] = PublicKey.findProgramAddressSync(
	[Buffer.from("collection"), mintCollection.toBuffer()],
	program.programId
);

async function initializeAcc(githubUsername: string): Promise<string | null> {
	try {
		const txhash = await program.methods
			.initialize(githubUsername)
			.accountsPartial({
				user: keypair.publicKey,
				account: account_key,
				system_program: SYSTEM_PROGRAM_ID,
			})
			.signers([keypair])
			.rpc();

		console.log(
			`Success! Check out your TX here : https://explorer.solana.com/tx/${txhash}?cluster=devnet`
		);

		return txhash;
	} catch (error) {
		console.log(`Oops something went wrong: ${error}`);
		return null;
	}
}

async function submitTS(): Promise<string | null> {
	try {
		const txhash = await program.methods
			.submitTs()
			.accountsPartial({
				user: keypair.publicKey,
				account: account_key,
				mint: mintTs.publicKey,
				collection: mintCollection,
				authority: authorityPda,
				mpl_core_program: MPL_CORE_PROGRAM_ID,
				system_program: SYSTEM_PROGRAM_ID,
			})
			.signers([keypair, mintTs])
			.rpc();

		console.log(
			`Success! Check out your TX here : https://explorer.solana.com/tx/${txhash}?cluster=devnet`
		);

		return txhash;
	} catch (error) {
		console.log(`Oops something went wrong: ${error}`);
		return null;
	}
}

(async function main() {
	console.log("Starting program...");

	const initResult = await initializeAcc("kirtiraj22");
	if (!initResult) {
		console.error("Aborting due to initialize error.");
		process.exit(1);
	}

	const submitResult = await submitTS();
	if (!submitResult) {
		console.error("Aborting due to submitTs error.");
		process.exit(1);
	}

	console.log("Done! All transactions completed successfully.");
})();
