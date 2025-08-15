use borsh::BorshDeserialize;
use Fractional_Marketplace::{
    instructions::{lock::LockNFTArgs},
    process_instruction
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn test_lock_nft() {
    // Program ID for your smart contract
    let program_id = Pubkey::new_unique();

    // Create test environment
    let mut program_test = ProgramTest::new(
        "Fractional-Marketplace", // name of your crate in Cargo.toml
        program_id,
        processor!(process_instruction),
    );

    // Start the test validator
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
}