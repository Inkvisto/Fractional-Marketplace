use mpl_utils::{
    assert_owner_in,
    assert_signer,
    cmp_pubkeys,
    token::SPL_TOKEN_PROGRAM_IDS
};
use {
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        msg,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        system_instruction,
        system_program,
        sysvar::Sysvar,
        rent::Rent,
        program_pack::Pack
    },
    spl_token::instruction as token_instruction
};

pub fn lock_nft(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Lock NFT");

    let account_info_iter = &mut accounts.iter();

    let user = next_account_info(account_info_iter)?; // NFT owner
    let user_nft_token_account = next_account_info(account_info_iter)?; // Source token account
    let pda_nft_token_account = next_account_info(account_info_iter)?; // Destination PDA token account
    let mint_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    // signers

    assert_signer(user)?;

    // ownership

    assert_owner_in(mint_account, &SPL_TOKEN_PROGRAM_IDS, ProgramError::IncorrectProgramId)?;

    // key match

    assert_keys_equal(system_program.key, &system_program::ID)?;
    assert_keys_equal(token_program.key, &spl_token::id())?;

    // Derive PDA
    let (pda, bump) = Pubkey::find_program_address(
        &[b"nft-lock", mint_account.key.as_ref()],
        program_id
    );

    if pda != *pda_nft_token_account.key {
        msg!("PDA account mismatch");
        return Err(ProgramError::InvalidArgument);
    }
    // Create PDA token account if it doesn't exist
    if pda_nft_token_account.data_is_empty() {
        let rent = &Rent::from_account_info(rent_sysvar)?;

        let space = spl_token::state::Account::LEN;
        let lamports = rent.minimum_balance(space);

        msg!("Create account");
        invoke_signed(
            &system_instruction::create_account(
                user.key,
                pda_nft_token_account.key,
                lamports,
                space as u64,
                token_program.key,
            ),
            &[
                user.clone(),
                pda_nft_token_account.clone(),
                system_program.clone(),
            ],
            &[&[b"nft-lock", mint_account.key.as_ref(), &[bump]]],
        )?;

        msg!("Initialize account");
        invoke(
            &token_instruction::initialize_account(
                token_program.key,
                pda_nft_token_account.key,
                mint_account.key,
                &pda,
            )?,
            &[
                pda_nft_token_account.clone(),
                mint_account.clone(),
                rent_sysvar.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("NFT transfer");
        // Transfer NFT (amount = 1)
        invoke(
            &token_instruction::transfer(
                token_program.key,
                user_nft_token_account.key,
                pda_nft_token_account.key,
                user.key,
                &[],
                1,
            )?,
            &[
                user_nft_token_account.clone(),
                pda_nft_token_account.clone(),
                user.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("NFT locked in PDA account");

    }

    Ok(())
}

pub fn assert_keys_equal(key1: &Pubkey, key2: &Pubkey) -> Result<(), ProgramError> {
    if !cmp_pubkeys(key1, key2) {
        // In Metaplex in this case the MetadataError::KeyMismatch error is being thrown
        // I tried to find something similar in ProgramError
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}
