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
		uri: "https://devnet.irys.xyz/5gyBVCAne8iR3PGXDjDCavdVs6VG6PsCqskLmksBxJBs",
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
// https://explorer.solana.com/tx/3DXKZyu6iACYy85iK2YSkSQsfrbwFgPN6QkqMZYJsxpmenxU1wdCFo2kivLM93edEHFSUbXyJY9giihrpvpqsZxX?cluster=devnet
// Mint Address:  HuENmSGVeE6t9NyEdMdn2QX5LxRGYmq3GaahwqVX6V2G
