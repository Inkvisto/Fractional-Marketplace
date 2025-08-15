// use borsh::BorshDeserialize;
// use Fractional_Marketplace::{
//     instructions::{lock::LockNFTArgs},
//     process_instruction
// };
// use solana_program_test::*;
// use solana_sdk::{
//     account::Account,
//     instruction::{AccountMeta, Instruction},
//     pubkey::Pubkey,
//     signature::Signer,
//     transaction::Transaction,
// };
//
// #[tokio::test]
// async fn test_lock_nft() {
//     // Program ID for your smart contract
//     let program_id = Pubkey::new_unique();
//
//     // Create test environment
//     let mut program_test = ProgramTest::new(
//         "helloworld", // name of your crate in Cargo.toml
//         program_id,
//         processor!(process_instruction),
//     );
//
//     // Start the test validator
//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
// }

use borsh::BorshDeserialize;
use Fractional_Marketplace::{process_instruction};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    println!("Zhopa");
}