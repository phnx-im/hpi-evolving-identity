use crate::types::EidError;
use crate::types::Member;
use crate::evolvement;
use crate::transcript::Transcript;

pub trait EidState: Sized {

    /// The type describing a step from one state to the next.
    type EvolvementProvider: evolvement::Evolvement;


    /// Create an [EidState] from a log of evolvements. Used to verify a slice of a transcript or to recover a state from a transcript.
    fn from_log(log: Vec<Self::EvolvementProvider>) -> Result<Self, EidError>;

    /// Apply an [evolvement::Evolvement], changing the [EidState]. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn apply(&mut self, evolvement: Self::EvolvementProvider) -> Result<(), EidError>;

    /// Verify that the current EID state is valid.
    fn verify(&self) -> Result<bool, EidError>;

    /// Verify that a client is part of the EID.
    fn verify_client(&self, client: Member) -> Result<bool, EidError>;

    /// Get all clients which are members of the EID.
    fn get_clients(&self) -> Result<Vec<Member>, EidError>;

}