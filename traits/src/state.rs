use crate::types::EidError;
use crate::types::Client;
use crate::evolvement;

pub trait EidState {

    type Evolvement: evolvement::Evolvement;

    fn apply(&self, evolvement: Self::Evolvement) -> Result<(), EidError>;

    fn verify(&self) -> Result(bool, EidError);

    fn add(&self) -> Result<Self::Evolvement, EidError>;

    fn remove(&self) -> Result<Self::Evolvement, EidError>;

    fn update(&self) -> Result<Self::Evolvement, EidError>;

    fn get_clients(&self) -> Result<Vec<Client>, EidError>;

}