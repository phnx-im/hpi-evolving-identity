use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::types::EidError;

pub trait EidState: Sized + Clone + Eq {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type BackendProvider: EidBackend;

    /// Create an [EidState] from a log of evolvements. Used to verify a slice of a transcript or to recover a state from a transcript.
    fn apply_log(
        &mut self,
        log: &[Self::EvolvementProvider],
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>
    where
        Self: Sized;

    /// Apply an [evolvement::Evolvement], changing the [EidState]. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn apply(
        &mut self,
        evolvement: &Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Verify that a client is part of the EID.
    fn verify_member(&self, member: &Self::MemberProvider) -> Result<bool, EidError>;

    /// Get all clients which are members of the EID.
    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError>;
}
