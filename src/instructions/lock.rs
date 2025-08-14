use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{AccountInfo},
        msg,
        entrypoint::ProgramResult,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LockNFTArgs {
    pub vault_bump: u8,
}

pub fn lock_nft(accounts: &[AccountInfo], args: LockNFTArgs) -> ProgramResult {
    msg!("Lock NFT with vault bump {}", args.vault_bump);

    Ok(())
}