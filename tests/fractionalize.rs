use std::str::FromStr;
use solana_program_test::*;
use solana_sdk::{
    instruction::{
        AccountMeta, 
        Instruction
    }, 
    pubkey::Pubkey, 
    signature::Signer, 
    transaction::Transaction
};
use solana_sdk::signature::{
    read_keypair_file, 
};
use Fractional_Marketplace::instructions::FractionalizeNFTArgs;
use Fractional_Marketplace::processor::FractionalMarketplaceInstruction;
use solana_client::rpc_client::RpcClient;
use crate::config::AppConfig;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_deployed() {
    let app_config = AppConfig::new("tests/config.toml")
        .expect("Config should be set");
    
    let client = RpcClient::new(app_config.solana.rpc_client_url);
    
    // Load payer
    let payer = read_keypair_file(app_config.solana.keypair_file_directory).unwrap();
    
    // Program ID from `solana program deploy`
    let program_id = Pubkey::from_str(&app_config.program.program_id).unwrap();
    
    let args = FractionalMarketplaceInstruction::Fractionalize(FractionalizeNFTArgs {
        total_shares: 7,
        nft_mint: Pubkey::new_unique(),
    });
    let data = borsh::to_vec(&args).unwrap();
    
    // For now, no extra accounts are needed (just payer)
    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![AccountMeta::new(payer.pubkey(), true)],
    );
    
    // Send transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    client.send_and_confirm_transaction(&tx).unwrap();
}