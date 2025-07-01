import wallet from "../turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createMetadataAccountV3,
	CreateMetadataAccountV3InstructionAccounts,
	CreateMetadataAccountV3InstructionArgs,
	DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
	createSignerFromKeypair,
	signerIdentity,
	publicKey,
} from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("48FGm8L3hQNzKNz2VD15bLvsZuWtmBPHtiaJoH8Ytnuf");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
	try {
		// Start here
		let accounts: CreateMetadataAccountV3InstructionAccounts = {
			mint,
			mintAuthority: signer,
		};

		let data: DataV2Args = {
			name: "TURBIN3",
			symbol: "TRBN3",
			uri: "https://img-cdn.magiceden.dev/da:t/rs:fill:400:0:0/plain/https%3A%2F%2Farweave.net%2FBijQf2vXXOt5y1ilcXvtFynj13CubQTm6GxgSDfPq7o",
			sellerFeeBasisPoints: 1,
			creators: null,
			collection: null,
			uses: null,
		};

		let args: CreateMetadataAccountV3InstructionArgs = {
			data,
			isMutable: true,
			collectionDetails: null,
		};

		let tx = createMetadataAccountV3(umi, {
			...accounts,
			...args,
		});

		let result = await tx.sendAndConfirm(umi);
		console.log(bs58.encode(result.signature));
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();

// 5chPr5c3LRyHgo27FSneYhQLCvdaVGxwiFURk32p3bgdJELzSosKQDyhiuR6DC1gh6je1ZLzFqJ4UpmQjabwe1ns
