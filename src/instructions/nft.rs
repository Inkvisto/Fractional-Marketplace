use {
    borsh::{BorshDeserialize, BorshSerialize}
};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer, Signature},
    transaction::Transaction,
    pubkey::Pubkey,
    commitment_config::{CommitmentConfig },
};
use mpl_core::instructions::CreateV1Builder;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NftMetadata {
    pub name: String,
    pub uri: String,
}

impl Default for NftMetadata {
    fn default() -> Self {
        Self {
            name: "My NFT".to_string(),
            uri: "https://example.com/my-nft.json".to_string(),
        }
    }
}

impl NftMetadata {
    pub fn new(name: String, uri: String) -> Self {
        Self {
            name,
            uri,
            ..Default::default()
        }
    }

}

/// Create a new NFT with customizable parameters
pub async fn create_asset(
    rpc_client: &RpcClient,
    metadata: NftMetadata,
    asset_keypair: Option<Keypair>,
    payer: &Keypair,
    additional_signers: &[&Keypair],
) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
    
    let asset = asset_keypair.unwrap_or_else(Keypair::new);

    let create_asset_ix = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .name(metadata.name)
        .uri(metadata.uri).instruction();



    // Combine all signers
    let mut signers = vec![payer, &asset];
    signers.extend(additional_signers);

    let last_blockhash = rpc_client.get_latest_blockhash().await?;

    let create_asset_tx = Transaction::new_signed_with_payer(
        &[create_asset_ix],
        Some(&payer.pubkey()),
        &signers,
        last_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&create_asset_tx).await?;

    Ok((asset.pubkey(), signature))
}

/// Create NFT with simple name and URI
pub async fn create_simple_nft(
    rpc_client: &RpcClient,
    name: &str,
    uri: &str,
    payer: &Keypair,
) -> Result<(Pubkey, Signature), Box<dyn std::error::Error>> {
    let metadata = NftMetadata::new(name.to_string(), uri.to_string());

    create_asset(rpc_client, metadata, None, payer, &[]).await
}


/// Check if NFT was created successfully
pub async fn verify_nft_creation(
    rpc_client: &RpcClient,
    asset_pubkey: &Pubkey,
) -> Result<bool, Box<dyn std::error::Error>> {
    let account_info = rpc_client
        .get_account_with_commitment(asset_pubkey, CommitmentConfig::confirmed())
        .await?;

    Ok(account_info.value.is_some())
}

/// Create multiple NFTs in batch
pub async fn create_batch_nfts(
    rpc_client: &RpcClient,
    nfts: Vec<(String, String)>,
    payer: &Keypair,
) -> Result<Vec<(Pubkey, Signature)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    for (name, uri) in nfts {
        match create_simple_nft(rpc_client, &name, &uri, payer).await {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("Failed to create NFT {}: {}", name, e),
        }
    }

    Ok(results)
}


