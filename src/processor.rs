use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    fractionalize::{fractionalize_nft, FractionalizeNFTArgs},
    lock::lock_nft,
    nft::{create_asset, create_simple_nft, verify_nft_creation, NftMetadata},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FractionalMarketplaceInstruction {
    Fractionalize(FractionalizeNFTArgs),
    Lock,
    Nft(NftMetadata),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FractionalMarketplaceInstruction::try_from_slice(instruction_data)?;

    match instruction {
        FractionalMarketplaceInstruction::Fractionalize(args) => fractionalize_nft(accounts, args),
        FractionalMarketplaceInstruction::Lock => lock_nft(program_id, accounts),
        FractionalMarketplaceInstruction::Nft(metadata) => {
            msg!("NFT creation instruction received: {:?}", metadata);
            Ok(())
        }
    }
}
