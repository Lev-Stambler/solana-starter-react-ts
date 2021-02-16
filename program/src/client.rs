//! Client library for compiling to WASM

use core::num;
use std::{borrow::Borrow, iter::FromIterator, ops::Add};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_utils::{pub_key_size, unpack_pubkey};
use wasm_bindgen::prelude::*;

use crate::{client_error::ClientError, instruction::ProgInstruction, solana_utils};

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will be processed as a
///     normal, unsigned spl-memo
///
#[wasm_bindgen]
pub fn call_instruction(
    instruction_json: &str,
    program_id_packed: Vec<u8>,
    pubkeys_packed: Vec<u8>,
    num_pubkeys: usize,
) {
    let instruction: ProgInstruction = serde_json::from_str(instruction_json).unwrap();
    call_instruction_inner(instruction, program_id_packed, pubkeys_packed, num_pubkeys).unwrap()
}

fn call_instruction_inner(
    instruction: ProgInstruction,
    program_id_packed: Vec<u8>,
    pubkeys_packed: Vec<u8>,
    num_pubkeys: usize,
) -> Result<(), ClientError> {
    if pubkeys_packed.len() != num_pubkeys * pub_key_size {
        return Err(ClientError::InvalidPublicKey);
    }

    let program_id = solana_utils::unpack_pubkey_corret_len(&program_id_packed)
        .map_err(|_| ClientError::InvalidProgramKey)?;
    // TODO: may be a faster way of doing this
    let pub_keys_res: Result<Vec<_>, ClientError> = pubkeys_packed
        .chunks_exact(pub_key_size)
        .into_iter()
        .map(|pub_key_packed| {
            solana_utils::unpack_pubkey_corret_len(pub_key_packed)
                .map(|ret| ret)
                .map_err(|_| ClientError::InvalidPublicKey)
        })
        .collect();
    // TODO: wasm_logging for errors
    let pub_keys = pub_keys_res?;
    build_instruction(&instruction, &pub_keys, &program_id);
    Ok(())
}

fn build_instruction(
    instr_data: &ProgInstruction,
    signer_pubkeys: &[Pubkey],
    program_id: &Pubkey,
) -> Instruction {
    // TODO: proper accounts given
    Instruction {
        program_id: program_id.to_owned(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey.borrow(), true))
            .collect(),
        data: instr_data.pack(),
    }
}
