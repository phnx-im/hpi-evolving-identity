//! # EID types

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Errors related to EID
#[derive(Debug)]
pub enum EidError {

}

impl Display for EidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for EidError {}

pub struct Client {
    pk: Vec<u8>,
}