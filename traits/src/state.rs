use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::types::EidError;

/// # EidState
/// The state of the EID. Each Client and the Transcript will have their own [EidState]
pub trait EidState: Sized {
    /// Type of [Evolvement](Self::EvolvementProvider) that can be applied on this [EidState](Self)
    type EvolvementProvider: Evolvement;

    /// Type of [Member](Self::MemberProvider) that EID holds.
    type MemberProvider: Member;

    /// Type of [EidBackend](Self::BackendProvider) this [EidState](Self) uses.
    type BackendProvider: EidBackend;

    /// Apply an [Evolvement] resulting in the next [EidState].
    ///
    /// # Arguments
    ///
    /// * `evolvement`: An [Evolvement]
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: Result<(), EidError> [EidError] if evolvement is invalid.
    ///
    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Verify that a [Member] is part of the EID.
    ///
    /// # Arguments
    ///
    /// * `member`: A potential [Member]
    ///
    /// returns: [bool]
    ///
    fn verify_member(&self, member: &Self::MemberProvider) -> bool {
        self.get_members().contains(member)
    }

    /// Get all [Member]s of the EID.
    /// returns: [Vec]<[Self::MemberProvider]>
    fn get_members(&self) -> Vec<Self::MemberProvider>;
}
