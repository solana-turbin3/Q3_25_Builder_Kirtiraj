import wallet from "../turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
	try {
		// Follow this JSON structure
		// https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
		const image =
			"https://devnet.irys.xyz/EiamsGBB5Ub4zfnaqcTcp3CFDtZgHLNsPh3Bmfy8vxUb";
		const metadata = {
			name: "ICE Berg",
			symbol: "ICEBERG",
			description: "Berg is a chill guy!",
			image: image,
			attributes: [{ trait_type: "rare", value: "10" }],
			properties: {
				files: [
					{
						type: "image/jpg",
						uri: image,
					},
				],
			},
			creators: [],
		};
		const myUri = await umi.uploader.uploadJson(metadata);
		console.log("Your metadata URI: ", myUri);
	} catch (error) {
		console.log("Oops.. Something went wrong", error);
	}
})();

// Your metadata URI(ft andre) : https://devnet.irys.xyz/A5EHC83vM5qUz1SGyUfmBE12VcWPVQtzTdut8eYrRofu
// Your metadata URI(ft berg):  https://arweave.net/7HNStfYTqcm1jqiyCdaimQf7n7e2ZKAULPFw5NhX3eQZ
