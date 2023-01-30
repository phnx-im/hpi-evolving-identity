use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::transcript::EidExportedTranscriptState;
use crate::transcript::EidTranscript;
use crate::types::EidError;

pub trait EidClient {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type TranscriptStateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;
    type ExportedTranscriptStateProvider: EidExportedTranscriptState<
        TranscriptStateProvider = Self::TranscriptStateProvider,
        BackendProvider = Self::BackendProvider,
    >;
    type BackendProvider: EidBackend;

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
        initial_member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create the [EidClient] with the state of an existing EID that you are invited to.
    fn create_from_invitation(
        invitation: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
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
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    fn cross_sign_membership(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>;

    /// Get all clients which are members of the EID.
    fn get_members(&self) -> Vec<Self::MemberProvider>;

    fn export_transcript_state(
        &self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::ExportedTranscriptStateProvider, EidError>;

    #[cfg(feature = "test")]
    fn generate_initial_id(id: String, backend: &Self::BackendProvider) -> Self::MemberProvider;
}
