use crate::types::EidError;
use crate::types::Client;
use crate::evolvement;

pub trait EidState {

    fn add() -> Result<(), EidError>;

    fn remove() -> Result<(), EidError>;

    fn update(&self) -> Result<Self::Evolvement, EidError>;

    fn get_clients(&self) -> Result<Vec<Client>, EidError>;

}