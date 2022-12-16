use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::types::EidError;

pub trait EidState<T: Evolvement, M: Member>: Sized + Clone + Eq {
    /// Create an [EidState] from a log of evolvements. Used to verify a slice of a transcript or to recover a state from a transcript.
    fn apply_log(&mut self, log: &Vec<T>) -> Result<(), EidError>
    where
        Self: Sized;

    /// Apply an [evolvement::Evolvement], changing the [EidState]. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn apply(&mut self, evolvement: &T) -> Result<(), EidError>;

    /// Verify that a client is part of the EID.
    fn verify_client(&self, client: &M) -> Result<bool, EidError>;

    /// Get all clients which are members of the EID.
    fn get_members(&self) -> Result<Vec<M>, EidError>;
}
