use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::types::EidError;

pub trait EidState: Sized {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type BackendProvider: EidBackend;

    /// Apply an [Evolvement], changing the [EidState]. If the [Evolvement]
    /// is invalid, return an [EidError].
    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Verify that a [Member] is part of the EID.
    fn verify_member(&self, member: &Self::MemberProvider) -> bool {
        self.get_members().contains(member)
    }

    /// Get all [Member]s of the EID.
    fn get_members(&self) -> Vec<Self::MemberProvider>;
}
