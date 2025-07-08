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

// const RPC_ENDPOINT = "https://api.devnet.solana.com";
const RPC_ENDPOINT = "https://devnet-rpc.shyft.to?api_key=Fki2F6DxrRTH3aeE";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
	console.log("enter");
	let tx = createNft(umi, {
		mint,
		name: "ICE Berg",
		symbol: "ICEBERG",
		uri: "https://devnet.irys.xyz/7HNStfYTqcm1jqiyCdaimQf7n7e2ZKAULPFw5NhX3eQZ",
		sellerFeeBasisPoints: percentAmount(5),
	});
	console.log("34");
	let result = await tx.sendAndConfirm(umi);
	console.log("38");
	const signature = base58.encode(result.signature);
	console.log("40");

	// console.log(
	// 	`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
	// );

	console.log("Mint Address: ", mint.publicKey);
})();

// Succesfully Minted! Check out your TX here:
// https://explorer.solana.com/tx/3mbsuYU6hmerJ22MkUbUfFijxJSxhDhpQKgwbeWewNhMDmpSaS5UgyEeQMcwW4BuE2AGzXoghNEJBxQrCg3Fw827?cluster=devnet
// Mint Address:  BVMnkMJD6gNMHwH9VkQ8Wv3aTd8ke1DCuEU4aw8j6Tzn
