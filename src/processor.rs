use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    fractionalize::{fractionalize_nft, FractionalizeNFTArgs},
    lock::{lock_nft, LockNFTArgs},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FractionalMarketplaceInstruction {
    Fractionalize(FractionalizeNFTArgs),
    Lock(LockNFTArgs)
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FractionalMarketplaceInstruction::try_from_slice(instruction_data)?;

    match instruction {
        FractionalMarketplaceInstruction::Fractionalize(args) => fractionalize_nft(accounts, args),
        FractionalMarketplaceInstruction::Lock(args) => lock_nft(program_id, accounts, args),
    }
}