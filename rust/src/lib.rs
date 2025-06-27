#[cfg(test)]
mod tests {
    // use super::*;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{ system_instruction::transfer };
    use solana_sdk::{
        signature::{ Keypair, Signer, read_keypair_file },
        pubkey::Pubkey,
        transaction::Transaction,
        hash::hash,
    };
    use std::str::FromStr;
    use bs58;
    use std::io::{ self, BufRead };

    //     const RPC_URL: &str =
    //         "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5
    // c9c80a";
    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string: ");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is :");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g [12, 33,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your base58-encoded private key is: ");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn claim_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Could't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here : ");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);

        let sig_hashed = hash(sig.as_ref());

        match sig.verify(pubkey.as_ref(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed!"),
        }

        let to_pubkey = Pubkey::from_str("BSgVj3bfNVoPNvWLGBn4LHaUwUoRSRS6H2NUP2GHXZKd").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction!");

        println!("Success! Check out your TX here : https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
}
