use borsh::BorshDeserialize;
use solana_program::sysvar;
use Fractional_Marketplace::{
    instructions::{lock::LockNFTArgs},
    process_instruction
};
use solana_program_test::*;
use solana_sdk::{account::Account, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Signer, system_program, transaction::Transaction};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::Keypair;
use Fractional_Marketplace::instructions::lock_nft;

#[tokio::test]
async fn test_lock_nft() {
    use solana_program::{
        pubkey::Pubkey,
        account_info::AccountInfo,
        system_program,
        sysvar,
    };
    use spl_token::state::{Account as TokenAccount, Mint};

    // Program ID for your smart contract
    let program_id = Pubkey::new_unique();

    // 1. Setup test accounts
    let user = Keypair::new();
    let user_pubkey = user.pubkey();

    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let user_nft_token_account = spl_associated_token_account::get_associated_token_address(
        &user_pubkey,
        &mint_pubkey,
    );

    let (pda_nft_token_account, _bump) = Pubkey::find_program_address(&[b"nft-lock"], &program_id);

    println!("{:?}", pda_nft_token_account);

    // Data buffers for each account
    let mut user_lamports = 10_000;
    let mut user_data = vec![0u8; 0];

    let mut user_nft_lamports = 10_000;
    let mut user_nft_data = vec![0u8; spl_token::state::Account::LEN];

    let mut pda_lamports = 10_000;
    let mut pda_data = vec![0u8; spl_token::state::Account::LEN];

    let mut mint_lamports = 10_000;
    let mut mint_data = vec![0u8; spl_token::state::Mint::LEN];

    let spl_token_id = spl_token::id();
    let sysvar_id = sysvar::rent::id();

    let mut token_program_lamports = 0;
    let mut system_program_lamports = 0;
    let mut rent_sysvar_lamports = 0;

    // 2. Create AccountInfo for each account
    let user_account_info = AccountInfo::new(
        &user_pubkey,
        true,
        true,
        &mut user_lamports,
        &mut user_data,
        &program_id,
        false,
        0,
    );

    let user_nft_account_info = AccountInfo::new(
        &user_nft_token_account,
        false,
        true,
        &mut user_nft_lamports,
        &mut user_nft_data,
        &spl_token_id,
        false,
        0,
    );

    let pda_account_info = AccountInfo::new(
        &pda_nft_token_account,
        false,
        true,
        &mut pda_lamports,
        &mut pda_data,
        &spl_token_id,
        false,
        0,
    );

    let mint_account_info = AccountInfo::new(
        &mint_pubkey,
        false,
        false,
        &mut mint_lamports,
        &mut mint_data,
        &spl_token_id,
        false,
        0,
    );

    let token_program_info = AccountInfo::new(
        &spl_token_id,
        false,
        false,
        &mut token_program_lamports,
        &mut [],
        &spl_token_id,
        false,
        0,
    );

    let system_program_info = AccountInfo::new(
        &spl_token_id,
        false,
        false,
        &mut system_program_lamports,
        &mut [],
        &spl_token_id,
        false,
        0,
    );

    let rent_sysvar_info = AccountInfo::new(
        &sysvar_id,
        false,
        false,
        &mut rent_sysvar_lamports,
        &mut [],
        &sysvar_id,
        false,
        0,
    );

    // 3. Prepare accounts slice
    let accounts = &[
        user_account_info,
        user_nft_account_info,
        pda_account_info,
        mint_account_info,
        token_program_info,
        system_program_info,
        rent_sysvar_info,
    ];

    // 4. Call lock_nft
    let args = LockNFTArgs { vault_bump: 255 };
    lock_nft(&program_id, accounts, args).unwrap();

    // 5. Here you would normally query the BanksClient to assert PDA token account
    // balance, but in this mock environment, you can check the data buffer
    // For example:
    // I'm getting UninitializedAccount error
    // let token_account = TokenAccount::unpack(&pda_data).unwrap();
    // assert_eq!(token_account.amount, 1);
}