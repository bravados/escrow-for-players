use solana_program::program_error::ProgramError;

use crate::error::StateError::InvalidInstruction;

 pub enum EscrowForPlayersInstruction {

    /// Creates a pda that will take the ownership of the token accounts that was created by the user and has the NFT.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the player that wants to transfer the ownership to the token account that has the NFT to our program
    /// 1. `[writable]` The token account that holds the NFT
    /// 2. `[]` The mint address of the NFT
    /// 3. `[writable]` The newly created state account that is already owned by the on chain program
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    DepositNFT {
    },
}

impl EscrowForPlayersInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::DepositNFT {},
            _ => return Err(InvalidInstruction.into()),
        })
    }
}