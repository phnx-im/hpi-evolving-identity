//! # EID types

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Errors related to EID
#[derive(Debug)]
pub enum EidError {
    DeserializationError(String),
    SerializationError(String),

    AddMemberError(String),
    UpdateMemberError(String),
    RemoveMemberError(String),

    InvalidMemberError(String),
    InvalidEvolvementError(String),
    InvalidInvitationError,

    CreateTranscriptStateError(String),
    ExportTranscriptStateError,
    ImportTranscriptStateError(String),

    CreateClientError(String),
    CreateCredentialError(String),
}

impl Display for EidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Error for EidError {}
