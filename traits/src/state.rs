use crate::types::EidError;

pub trait EidState {

    fn add() -> Result<(), EidError>;

    fn remove() -> Result<(), EidError>;

    fn update() -> Result<(), EidError>;
}