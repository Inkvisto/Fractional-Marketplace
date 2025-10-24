use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{AccountInfo},
        msg,
        pubkey::Pubkey,
        entrypoint::ProgramResult,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FractionalizeNFTArgs {
    pub nft_mint: Pubkey,
    pub total_shares: u64
}

pub fn fractionalize_nft(_accounts: &[AccountInfo], args: FractionalizeNFTArgs) -> ProgramResult {
    msg!("fractionalize_nft with pubkey {} to such number of shares: {}", args.nft_mint, args.total_shares);

    Ok(())
}