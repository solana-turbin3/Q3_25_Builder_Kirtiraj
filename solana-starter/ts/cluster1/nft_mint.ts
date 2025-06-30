import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createSignerFromKeypair,
	signerIdentity,
	generateSigner,
	percentAmount,
} from "@metaplex-foundation/umi";
import {
	createNft,
	mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
	let tx = createNft(umi, {
		mint,
		name: "RUGDAY",
		symbol: "RGDAY",
		uri: "https://devnet.irys.xyz/VRc2Wi1HC2kVgB4mgWfrfs4xrmfaynXQDXmHbAe2AZC",
		sellerFeeBasisPoints: percentAmount(5),
	});
	let result = await tx.sendAndConfirm(umi);
	const signature = base58.encode(result.signature);

	console.log(
		`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
	);

	console.log("Mint Address: ", mint.publicKey);
})();

// Succesfully Minted! Check out your TX here:
// https://explorer.solana.com/tx/3TvVi6u4E1EtN1ARdnDRhdgALemxrPuq3NEaHqrwQYxwgh38iP9y6hHhUR39jiKr5ks4F2JmFYLWoHEX1zBPY12K?cluster=devnet
// Mint Address:  BRAGnMmBnqZVxgsoGMoB3naE3ChjazY5ZDUrWDTbjU2i
