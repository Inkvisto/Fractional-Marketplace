use std::str::FromStr;
use spl_associated_token_account::{get_associated_token_address, processor};
use solana_program_test::*;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Signer, system_instruction, system_program, transaction::Transaction};
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::{read_keypair_file, Keypair};
use Fractional_Marketplace::instructions::FractionalizeNFTArgs;
use Fractional_Marketplace::processor::FractionalMarketplaceInstruction;
use solana_client::rpc_client::RpcClient;
use spl_token::{
    instruction as token_instruction,
    state::Mint,
    ID as TOKEN_PROGRAM_ID,
};
use crate::PROGRAM_ID;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_deployed() {
    let client = RpcClient::new("http://127.0.0.1:8899".to_string());

    // Load payer
    let payer = read_keypair_file("/home/misha/.config/solana/id.json").unwrap();

    // Program ID from `solana program deploy`
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

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