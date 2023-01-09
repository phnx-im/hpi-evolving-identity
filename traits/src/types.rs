//! # EID types

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Errors related to EID
#[derive(Debug)]
pub enum EidError {
    StateNotInitialized,
    StateAlreadyInitialized,
    AddMemberError(String),
    InvalidMemberError(String),
    ParseMessageError,
    UnverifiedMessageError,
    InvalidMessageError,
    ApplyCommitError,
    UpdateMemberError(String),
}

impl Display for EidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Error for EidError {}

pub enum EvolvementType {
    Add,
    Update,
    Remove,
}
