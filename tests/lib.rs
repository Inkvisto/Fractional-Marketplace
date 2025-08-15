use Fractional_Marketplace::{
    instructions::{lock::LockNFTArgs},
};
use solana_program_test::*;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Signer, system_instruction, system_program, transaction::Transaction};
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::Keypair;

#[tokio::test]
async fn test_lock_nft() {
    use solana_program::{
        pubkey::Pubkey,
        system_program,
    };
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        instruction::{Instruction, AccountMeta},
    };
    use spl_token::state::Account as TokenAccount;
    use spl_associated_token_account;

    // Program ID
    let program_id = Pubkey::new_unique();

    // User & Mint
    let user = Keypair::new();
    let user_pubkey = user.pubkey();
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();

    // Start ProgramTest
    let mut program_test = ProgramTest::new(
        "Fractional-Marketplace",
        program_id,
        processor!(Fractional_Marketplace::processor::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // 1️⃣ Create mint account
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);
    let create_mint_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &user_pubkey,
        None,
        0,
    )
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_mint_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // 2️⃣ Create user's associated token account
    let user_nft_token_account =
        spl_associated_token_account::get_associated_token_address(&user_pubkey, &mint_pubkey);

    let create_user_token_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &user_pubkey,
            &mint_pubkey,
            &spl_token::id()
        );

    let tx = Transaction::new_signed_with_payer(
        &[create_user_token_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // 3️⃣ Mint 1 NFT to user
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &user_nft_token_account,
        &user_pubkey,
        &[],
        1,
    )
        .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer, &user],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // --- FUND USER ACCOUNT ---
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(spl_token::state::Account::LEN); // enough for PDA token account

    let fund_user_ix = solana_program::system_instruction::transfer(
        &payer.pubkey(),
        &user_pubkey,
        lamports,
    );

    let tx = Transaction::new_signed_with_payer(
        &[fund_user_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // 4️⃣ Derive PDA
    let (pda_nft_token_account, _bump) = Pubkey::find_program_address(&[b"nft-lock"], &program_id);

    // 5️⃣ Prepare lock_nft instruction
    let args = LockNFTArgs {};
    let data = borsh::to_vec(&args).unwrap();

    let accounts = vec![
        AccountMeta::new(user_pubkey, true), // signer
        AccountMeta::new(user_nft_token_account, false),
        AccountMeta::new(pda_nft_token_account, false),
        AccountMeta::new_readonly(mint_pubkey, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
    ];

    let lock_ix = Instruction {
        program_id,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[lock_ix],
        Some(&payer.pubkey()),
        &[&payer, &user],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // 6️⃣ Fetch PDA token account and check amount
    let pda_account = banks_client
        .get_account(pda_nft_token_account)
        .await
        .unwrap()
        .expect("PDA account not found");

    let token_account = TokenAccount::unpack(&pda_account.data).unwrap();
    assert_eq!(token_account.amount, 1);
}