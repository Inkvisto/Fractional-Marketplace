use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        msg,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        system_instruction,
        sysvar::Sysvar,
        rent::Rent,
        program_pack::Pack
    },
    spl_token::instruction as token_instruction
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LockNFTArgs {
    pub vault_bump: u8,
}

pub fn lock_nft(program_id: &Pubkey, accounts: &[AccountInfo], args: LockNFTArgs) -> ProgramResult {
    msg!("Lock NFT with vault bump {}", args.vault_bump);

    // let account_info_iter = &mut accounts.iter();
    //
    // let user = next_account_info(account_info_iter)?; // NFT owner
    // let user_nft_token_account = next_account_info(account_info_iter)?; // Source token account
    // let pda_nft_token_account = next_account_info(account_info_iter)?; // Destination PDA token account
    // let mint_account = next_account_info(account_info_iter)?;
    // let token_program = next_account_info(account_info_iter)?;
    // let system_program = next_account_info(account_info_iter)?;
    // let rent_sysvar = next_account_info(account_info_iter)?;
    //
    // // Derive PDA
    // let (pda, bump) = Pubkey::find_program_address(&[b"nft-lock"], program_id);
    //
    // if pda != *pda_nft_token_account.key {
    //     msg!("PDA account mismatch");
    //     return Err(ProgramError::InvalidArgument);
    // }
    //
    // // Create PDA token account if it doesn't exist
    // if pda_nft_token_account.data_is_empty() {
    //     let rent = Rent::get()?;
    //     let space = spl_token::state::Account::LEN;
    //     let lamports = rent.minimum_balance(space);
    //
    //     invoke_signed(
    //         &system_instruction::create_account(
    //             user.key,
    //             pda_nft_token_account.key,
    //             lamports,
    //             space as u64,
    //             token_program.key,
    //         ),
    //         &[
    //             user.clone(),
    //             pda_nft_token_account.clone(),
    //             system_program.clone(),
    //         ],
    //         &[&[b"nft-lock", &[bump]]],
    //     )?;
    //
    //     invoke(
    //         &token_instruction::initialize_account(
    //             token_program.key,
    //             pda_nft_token_account.key,
    //             mint_account.key,
    //             &pda,
    //         )?,
    //         &[
    //             pda_nft_token_account.clone(),
    //             mint_account.clone(),
    //             rent_sysvar.clone(),
    //             token_program.clone(),
    //         ],
    //     )?;
    //
    //     // Transfer NFT (amount = 1)
    //     invoke(
    //         &token_instruction::transfer(
    //             token_program.key,
    //             user_nft_token_account.key,
    //             pda_nft_token_account.key,
    //             user.key,
    //             &[],
    //             1,
    //         )?,
    //         &[
    //             user_nft_token_account.clone(),
    //             pda_nft_token_account.clone(),
    //             user.clone(),
    //             token_program.clone(),
    //         ],
    //     )?;
    //
    //     msg!("NFT locked in PDA account");
    //
    // }

    Ok(())
}