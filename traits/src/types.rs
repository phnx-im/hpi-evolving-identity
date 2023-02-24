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
    InvalidInvitationError(String),

    CreateTranscriptStateError(String),
    ExportTranscriptStateError(String),
    ImportTranscriptStateError(String),

    CreateClientError(String),
    CreateCredentialError(String),
}

impl Display for EidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_string = match &self {
            EidError::DeserializationError(s)
            | EidError::SerializationError(s)
            | EidError::AddMemberError(s)
            | EidError::UpdateMemberError(s)
            | EidError::RemoveMemberError(s)
            | EidError::InvalidMemberError(s)
            | EidError::InvalidEvolvementError(s)
            | EidError::InvalidInvitationError(s)
            | EidError::CreateTranscriptStateError(s)
            | EidError::ExportTranscriptStateError(s)
            | EidError::ImportTranscriptStateError(s)
            | EidError::CreateClientError(s)
            | EidError::CreateCredentialError(s) => s,
        };
        write!(f, "{:?}", error_string)
    }
}

impl Error for EidError {}
