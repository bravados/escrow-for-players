use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},

};

use spl_token::state::Account as TokenAccount;

use crate::{instruction::EscrowForPlayersInstruction, error::StateError, state::State};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = EscrowForPlayersInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowForPlayersInstruction::DepositNFT {} => {
                msg!("Instruction: DepositNFT");
                Self::process_deposit_nft(accounts, program_id)
            },
        }
    }

    fn process_deposit_nft(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("Starting the deposit of the nft");

        let account_info_iter = &mut accounts.iter();
        let depositor = next_account_info(account_info_iter)?;

        if !depositor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let token_account_with_nft_account_info = next_account_info(account_info_iter)?;
        if *token_account_with_nft_account_info.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        msg!("Checking the mint of the token account provided");
        let mint_account_info = next_account_info(account_info_iter)?;
        let token_account_with_nft_token_account = TokenAccount::unpack(&token_account_with_nft_account_info.data.borrow())?;
        if token_account_with_nft_token_account.mint != *mint_account_info.key {
            return Err(StateError::NFTMismatchMint.into());
        }

        msg!("Checking the amount of the NFT. Must be one");
        if token_account_with_nft_token_account.amount != 1000000000 {
            msg!("The amount is: {}", token_account_with_nft_token_account.amount);
            return Err(StateError::ExpectedAmountMismatch.into());
        }

        let state_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        msg!("Checking if the state account is rent exempt");
        if !rent.is_exempt(state_account.lamports(), state_account.data_len()) {
            return Err(StateError::NotRentExempt.into());
        }

        msg!("Writing in the state account");
        msg!("len of the coming state account: {}", &state_account.data.borrow().len());
        let mut state_info = State::unpack_unchecked(&state_account.data.borrow())?;

        state_info.is_available = true;
        state_info.previous_owner_pubkey = *depositor.key;

        msg!("Packing everything into the state account");
        State::pack(state_info, &mut state_account.data.borrow_mut())?;


        msg!("Create pda");
        let (pda, _bump_seed) = Pubkey::find_program_address(&[&depositor.key.to_bytes(), &mint_account_info.key.to_bytes()], program_id);

        msg!("Creating the Ix to change the ownership of the token account");
        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            token_account_with_nft_account_info.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            depositor.key,
            &[&depositor.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                token_account_with_nft_account_info.clone(),
                depositor.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    
}