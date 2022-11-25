//! # EID types

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Errors related to EID
#[derive(Debug)]
pub enum EidError {
    StateNotInitialized,
    StateAlreadyInitialized,
}

impl Display for EidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for EidError {}

#[derive(Clone, Default)]
pub struct Member {
    pk: Vec<u8>,
}

impl Member {
    pub fn pk(&self) -> Vec<u8> {
        self.pk.clone()
    }
}
