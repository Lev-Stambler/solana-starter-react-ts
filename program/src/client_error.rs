//! Client errors for the WASM bindgen

use num_derive::FromPrimitive;
use thiserror::Error;

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ClientError {
    #[error("Invalid or public key provided")]
    InvalidPublicKey,
    #[error("Invalid program key")]
    InvalidProgramKey,
}
