use crate::evolvement::Evolvement;
use crate::types::EidError;
use crate::types::Member;

pub trait EidState<T: Evolvement>: Sized + Clone + Eq {
    /// Create an [EidState] from a log of evolvements. Used to verify a slice of a transcript or to recover a state from a transcript.
    fn from_log(log: &[T]) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Apply an [evolvement::Evolvement], changing the [EidState]. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn apply(&mut self, evolvement: &T) -> Result<(), EidError>;

    /// Verify that the current EID state is valid.
    fn verify(&self) -> Result<bool, EidError>;

    /// Verify that a client is part of the EID.
    fn verify_client(&self, client: &Member) -> Result<bool, EidError>;

    /// Get all clients which are members of the EID.
    fn get_members(&self) -> Result<Vec<Member>, EidError>;
}
