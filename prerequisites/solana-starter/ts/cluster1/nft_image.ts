import wallet from "../turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
	try {
		//1. Load image
		const image = await readFile("./iceberg.jpg");
		//2. Convert image to generic file.
		const genericFile = createGenericFile(image, "rugged", {
			contentType: "image/jpg",
		});
		//3. Upload image
		const [myUri] = await umi.uploader.upload([genericFile]);
		console.log("Your image URI: ", myUri);
	} catch (error) {
		console.log("Oops.. Something went wrong", error);
	}
})();

// Your image URI(ft. andre):  https://arweave.net/3VzsARrwvCy8ePstyy5HjfZspYggyYSMi91U7qdSrLYk
// Your image URI(ft. berg):  https://arweave.net/EiamsGBB5Ub4zfnaqcTcp3CFDtZgHLNsPh3Bmfy8vxUb
