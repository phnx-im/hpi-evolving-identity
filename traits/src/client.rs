use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::types::EidError;

pub trait EidClient {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type StateProvider: EidState<Self::EvolvementProvider, Self::MemberProvider>;
    type BackendProvider;

    fn state(&self) -> &Self::StateProvider;

    fn pk(&self) -> &[u8];

    /// Create the first [EidState] of an EID by interacting with a PKI. We assume trust on first use on the resulting [EidState].
    fn create_eid(backend: &Self::BackendProvider) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to add a member to the EID.
    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to remove a member from the EID.
    fn remove(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to update your own key material.
    fn update(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>;

    /// Apply an [evolvement::Evolvement], changing the client's state. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn evolve(
        &mut self,
        evolvement: &Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;
}
