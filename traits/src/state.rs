use crate::types::EidError;
use crate::types::Client;
use crate::evolvement;

pub trait EidState {

    /// The type describing a step from one state to the next.
    type Evolvement: evolvement::Evolvement;

    /// Apply an [evolvement::Evolvement], changing the [EidState]. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn apply(&self, evolvement: Self::Evolvement) -> Result<(), EidError>;

    /// Verify that the current EID state is valid.
    fn verify(&self) -> Result(bool, EidError);

    /// Verify that a client is part of the EID.
    fn verify_client(&self, client: Client) -> Result<bool, EidError>;

    /// Get all clients which are members of the EID.
    fn get_clients(&self) -> Result<Vec<Client>, EidError>;

}