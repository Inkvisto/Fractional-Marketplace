use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair_file};
use Fractional_Marketplace::instructions::{NftMetadata, create_simple_nft, create_asset, verify_nft_creation};
use std::time::Duration;
use solana_program_test::*;

#[tokio::test]
async fn test_create_simple_nft() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let payer = read_keypair_file("/home/egor/.config/solana/id.json").unwrap();

    if rpc_client.get_latest_blockhash().await.is_err() {
        println!("Skipping test - no local validator running");
        return;
    }


    match create_simple_nft(
        &rpc_client,
        "Test Simple NFT",
        "https://example.com/simple-test.json",
        &payer,
    ).await {
        Ok((asset_pubkey, signature)) => {
            println!("Simple NFT test passed!");
            println!("Asset: {}", asset_pubkey);
            println!("Signature: {}", signature);
            
            // Verify creation
            tokio::time::sleep(Duration::from_secs(2)).await;
            let exists = verify_nft_creation(&rpc_client, &asset_pubkey).await.unwrap_or(false);
            assert!(exists, "NFT should exist on chain");
        }
        Err(e) => {
            if e.to_string().contains("connection") {
                println!("Connection error - test environment issue");
            } else {
                panic!("Simple NFT test failed: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_create_custom_nft() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let payer = read_keypair_file("/home/egor/.config/solana/id.json").unwrap();

    if rpc_client.get_latest_blockhash().await.is_err() {
        println!("Skipping custom test - no local validator running");
        return;
    }


    let custom_metadata = NftMetadata {
        name: "Custom Test NFT".to_string(),
        uri: "https://arweave.net/custom-test.json".to_string(),
    };

    match create_asset(&rpc_client, custom_metadata, None, &payer, &[]).await {
        Ok((asset_pubkey, signature)) => {
            println!("Custom NFT test passed!");
            println!("Asset: {}", asset_pubkey);
            println!("Signature: {}", signature);
        }
        Err(e) => {
            if e.to_string().contains("connection") {
                println!("Connection error - test environment issue");
            } else {
                panic!("Custom NFT test failed: {}", e);
            }
        }
    }
}

