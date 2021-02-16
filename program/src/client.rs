use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_utils::unpack_pubkey;
use wasm_bindgen::prelude::*;

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will be processed as a
///     normal, unsigned spl-memo
///
#[wasm_bindgen]
pub fn build_memo(memo: &[u8], pubkey_packed: &[u8]) -> Instruction {
    let pubkey = solana_utils::unpack_pubkey_corret_len(pubkey_packed)?;
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}
