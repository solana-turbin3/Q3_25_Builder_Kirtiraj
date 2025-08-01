import {
	Commitment,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
} from "@solana/web3.js";
import wallet from "../turbin3-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("48FGm8L3hQNzKNz2VD15bLvsZuWtmBPHtiaJoH8Ytnuf");

// Recipient address
const to = new PublicKey("9jAHeHugriZMScDcmsiJkP9CDDpuYEoSeGs5LHchtY15");

(async () => {
	try {
		// Get the token account of the fromWallet address, and if it does not exist, create it
		const fromWallet = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);
		// Get the token account of the toWallet address, and if it does not exist, create it
		const toWallet = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			to
		);
		// Transfer the new token to the "toTokenAccount" we just created
		const tx = await transfer(
			connection,
			keypair,
			fromWallet.address,
			toWallet.address,
			keypair,
			1000
		);

		console.log("Tokens transferred successfully: ", tx);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();

// Tokens transferred successfully:  96z7ikz6SXVcrkFTAE9xPdniPrfr74J6xyxZERFqr2Z4rnFYbZwCqHjaps3wS5nyYNHFvfsFcvCXRzsqVBKfj96
