//! # EID types

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Errors related to EID
#[derive(Debug)]
pub enum EidError {
    StateNotInitialized,
    StateAlreadyInitialized,
    AddMemberError(String),
    DeserializationError(String),
    RemoveMemberError(String),
    InvalidMemberError(String),
    ProcessMessageError(String),
    UnverifiedMessageError,
    InvalidMessageError(String),
    ApplyCommitError(String),
    UpdateMemberError(String),
    ExportGroupInfoError,
    SerializationError(String),
    InvalidInvitationError,
    CreateGroupError(String),
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
