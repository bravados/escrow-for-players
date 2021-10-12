use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct State {
    pub is_available: bool,
    pub previous_owner_pubkey: Pubkey,
}

impl Sealed for State {}

impl Pack for State {
    const LEN: usize = 33;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, State::LEN];
        let (
            is_available,
            previous_owner_pubkey,
        ) = array_refs![src, 1, 32];

        let is_available = match is_available {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(State {
            is_available,
            previous_owner_pubkey: Pubkey::new_from_array(*previous_owner_pubkey),
        })
    }
    
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, State::LEN];
        let (
            is_available_dst,
            previous_owner_pubkey_dst,
        ) = mut_array_refs![dst, 1, 32];

        let State {
            is_available,
            previous_owner_pubkey
        } = self;

        is_available_dst[0] = *is_available as u8;
        previous_owner_pubkey_dst.copy_from_slice(previous_owner_pubkey.as_ref());
    }
}

