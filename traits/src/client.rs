use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::transcript::EidTranscript;
use crate::types::EidError;

pub trait EidClient {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type TranscriptStateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;
    type BackendProvider: EidBackend;
    type InitialIdentityProvider;

    // We're only requiring this for tests since we don't want to unnecessarily restrict the transcript type.
    #[cfg(feature = "test")]
    type TranscriptProvider: EidTranscript<
        EvolvementProvider = Self::EvolvementProvider,
        StateProvider = Self::TranscriptStateProvider,
        BackendProvider = Self::BackendProvider,
        MemberProvider = Self::MemberProvider,
    >;

    /// Create the first [EidState] of an EID by interacting with a PKI. We assume trust on first use on the resulting [EidState].
    fn create_eid(
        identity: Self::InitialIdentityProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to add a member to the EID.
    fn add(
        &mut self,
        identity: Self::InitialIdentityProvider,
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
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Get all clients which are members of the EID.
    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError>;

    fn export_transcript_state(
        &self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::TranscriptStateProvider, EidError>;

    #[cfg(feature = "test")]
    fn generate_initial_id(backend: &Self::BackendProvider) -> Self::InitialIdentityProvider;
}
