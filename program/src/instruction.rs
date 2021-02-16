//! Instruction types

// use crate::error::TokenError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar,
};
use std::convert::TryInto;
use std::mem::size_of;

/// Minimum number of multisignature signers (min N)
pub const MIN_SIGNERS: usize = 1;
/// Maximum number of multisignature signers (max N)
pub const MAX_SIGNERS: usize = 11;

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum ProgInstruction {
}

impl ProgInstruction {
    /// Unpacks a byte buffer into a [ProgInstruction](enum.ProgInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

    }

    /// Packs a [ProgInstruction](enum.ProgInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {

    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_packing() {

    }
}