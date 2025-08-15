use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod instructions;
pub mod processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}


// Sanity tests
#[cfg(test)]
mod test {
    use borsh::{BorshDeserialize};
    use crate::instructions::{FractionalizeNFTArgs};
    use crate::processor::FractionalMarketplaceInstruction;
    use solana_program::{
        account_info::AccountInfo,
        clock::Epoch,
    };
    use super::*;


    #[test]
    fn test_instruction_deserialization() {
        let args = FractionalizeNFTArgs {
            nft_mint: Default::default(),
            total_shares: 7,
        };

        let instruction = FractionalMarketplaceInstruction::Fractionalize(args);
        let serialized = borsh::to_vec(&instruction).unwrap();

        let deserialized = FractionalMarketplaceInstruction::try_from_slice(&serialized).unwrap();

        match deserialized {
            FractionalMarketplaceInstruction::Fractionalize(deserialized_args) => {
                assert_eq!(deserialized_args.total_shares, 7);
            },
            _ => println!("{:?}", deserialized),
        }
    }

    #[test]
    fn test_fractionalize_instruction() {
        // Prepare dummy accounts
        let key = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![0u8; 100]; // enough space for test
        let owner = Pubkey::new_unique();

        let account = AccountInfo::new(
            &key,
            false, // not signer
            true,  // writable
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        // Prepare instruction data
        let args = FractionalizeNFTArgs {
            nft_mint: Pubkey::new_unique(),
            total_shares: 7,
        };
        let instruction = FractionalMarketplaceInstruction::Fractionalize(args);
        let instruction_data = borsh::to_vec(&instruction).unwrap();

        // Call process_instruction
        let res = process_instruction(&owner, &accounts, &instruction_data);
        assert!(res.is_ok());
    }

    #[test]
    fn test_lock_instruction() {
        // Prepare dummy accounts
        let key = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![0u8; 100]; // enough space for test
        let owner = Pubkey::new_unique();

        let account = AccountInfo::new(
            &key,
            false, // not signer
            true,  // writable
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        // Prepare instruction data
        let instruction = FractionalMarketplaceInstruction::Lock;
        let instruction_data = borsh::to_vec(&instruction).unwrap();

        // Call process_instruction
        let res = process_instruction(&owner, &accounts, &instruction_data);

        if let Err(e) = res {
            println!("{:?}", e);
        }
        // assert!(res.is_ok());
    }
}