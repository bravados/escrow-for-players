use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StateError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exent")]
    NotRentExempt,
    #[error("NFT Mismach Mint of Token Account")]
    NFTMismatchMint,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch
}

impl From<StateError> for ProgramError {
    fn from(e: StateError) -> Self {
        ProgramError::Custom(e as u32)
    }
}