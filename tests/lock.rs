use mpl_core::Asset;
use {
    solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction
    },
    solana_client::rpc_client::RpcClient,
    crate::config::AppConfig,
    mpl_core::instructions::CreateV1Builder,
    std::str::FromStr,
    solana_program::{
        instruction::{Instruction, AccountMeta},
        pubkey::Pubkey,
        system_program::ID as SYSTEM_PROGRAM_ID,
    },
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_deployed() {
    let app_config = AppConfig::new("tests/config.toml")
        .expect("Config should be set");

    let client = RpcClient::new(app_config.solana.rpc_client_url);

    let payer = read_keypair_file(app_config.solana.keypair_file_directory)
        .expect("Failed to read keypair file");

    let asset = Keypair::new();
    let collection = Keypair::new();

    // 1️⃣ Create the NFT
    let create_ix = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .update_authority(Some(payer.pubkey()))
        .owner(Some(payer.pubkey()))
        .name("My Test NFT".to_string())
        .uri("https://example.com/nft.json".to_string())
        .instruction();

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx_create = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer, &asset],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&tx_create).unwrap();

    // 2️⃣ Lock the NFT into PDA
    let program_id = Pubkey::from_str(&app_config.program.program_id)
        .expect("Failed to create program id");

    let mpl_core_program_id = Pubkey::from_str(&app_config.mpl_core_program_id)
        .expect("Failed to create mpl_core_program_id");

    let lock_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),      // user
            AccountMeta::new(asset.pubkey(), false),     // asset account
            AccountMeta::new(collection.pubkey(), false),// collection
            AccountMeta::new_readonly(mpl_core_program_id, false), // mpl core
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false), // system program
        ],
        data: vec![],
    };

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx_lock = Transaction::new_signed_with_payer(
        &[lock_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&tx_lock).unwrap();

    println!("NFT created and locked successfully!");

    // 3️⃣ Fetch the asset account and assert ownership
    let asset_account_data = client
        .get_account(&asset.pubkey())
        .expect("Asset account not found");

    // Unpack the SPL token account
    let asset_state = *Asset::deserialize(&mut asset_account_data.data.as_slice())
        .expect("Failed to deserialize MPL Core asset");

    let (pda, _) = Pubkey::find_program_address(
        &[b"nft-lock", asset.pubkey().as_ref()],
        &program_id
    );

    assert_eq!(asset_state.base.owner, pda, "Asset should be owned by PDA after lock");
}