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

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Member {
    pk: Vec<u8>,
}

impl Member {
    pub fn new(pk: Vec<u8>) -> Self {
        Self { pk }
    }
    pub fn pk(&self) -> Vec<u8> {
        self.pk.clone()
    }
}
