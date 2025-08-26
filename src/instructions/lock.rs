use {
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        msg,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        program::{invoke},
        program_error::ProgramError,
        system_program::ID as SYSTEM_PROGRAM_ID,
    },
    mpl_utils::{
        assert_signer,
        cmp_pubkeys,
    },
    mpl_core::{
        instructions::{TransferV1Builder},
        ID as MPL_CORE_ID
    }
};

pub fn lock_nft(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Lock NFT using MPL Core");

    let account_info_iter = &mut accounts.iter();

    let user = next_account_info(account_info_iter)?; // Current owner/authority
    let asset_account = next_account_info(account_info_iter)?; // MPL Core Asset account
    let collection_account = next_account_info(account_info_iter)?; // Collection (optional)
    let mpl_core_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // signers

    assert_signer(user)?;

    // key match

    assert_keys_equal(mpl_core_program.key, &MPL_CORE_ID)?;
    assert_keys_equal(system_program.key, &SYSTEM_PROGRAM_ID)?;

    // Derive PDA

    let (lock_authority_pda, bump) = Pubkey::find_program_address(
        &[b"nft-lock", asset_account.key.as_ref()],
        program_id
    );

    let transfer_ix = TransferV1Builder::new()
        .asset(*asset_account.key)
        .collection(Some(*collection_account.key))
        .payer(*user.key)
        .authority(Some(*user.key))
        .new_owner(lock_authority_pda)
        .instruction();

    invoke(
        &transfer_ix,
        &[
            asset_account.clone(),
            collection_account.clone(),
            user.clone(),
            mpl_core_program.clone(),
            system_program.clone(),
        ],
    )?;

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
