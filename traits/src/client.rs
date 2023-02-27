#[cfg(feature = "test")]
use std::fmt::Debug;

use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::transcript::EidExportedTranscriptState;
#[cfg(feature = "test")]
use crate::transcript::EidTranscript;
use crate::types::EidError;

/// # EidClient
/// A Client of an EID that can create new [Evolvement]s and evolve its [EidState] by applying any [Evolvement]s.
pub trait EidClient {
    /// Type of [Evolvement](Self::EvolvementProvider)s the client creates.
    #[cfg(not(feature = "test"))]
    type EvolvementProvider: Evolvement;
    /// Type of [Evolvement](Self::EvolvementProvider)s the client creates.
    #[cfg(feature = "test")]
    type EvolvementProvider: Evolvement + Debug;
    /// Type of [Member](Self::MemberProvider)s that can be added or removed from the EID.
    type MemberProvider: Member;

    /// The type of [Transcript State](Self::TranscriptStateProvider) this corresponds to.
    type TranscriptStateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;

    /// Type of [State](Self::ExportedTranscriptStateProvider) that can be exported from a client and is used to create a corresponding transcript state.
    type ExportedTranscriptStateProvider: EidExportedTranscriptState<
        TranscriptStateProvider = Self::TranscriptStateProvider,
        BackendProvider = Self::BackendProvider,
    >;
    /// Type of [EidBackend](Self::BackendProvider) this [EidClient](Self) uses.
    type BackendProvider: EidBackend;
    type KeyProvider;

    // We're only requiring this for tests since we don't want to unnecessarily restrict the transcript type.
    #[cfg(feature = "test")]
    type TranscriptProvider: EidTranscript<
        EvolvementProvider = Self::EvolvementProvider,
        StateProvider = Self::TranscriptStateProvider,
        BackendProvider = Self::BackendProvider,
        MemberProvider = Self::MemberProvider,
    >;

    /// Create the first [EidClient](Self) of an EID including the first [EidState] of an EID.
    /// We assume trust on first use on the resulting [EidState].
    ///
    /// # Arguments
    ///
    /// * `initial_member`: The first [Member](Self::MemberProvider)
    /// * `key_pair`: The [Member](Self::MemberProvider)'s key material
    /// * `backend`: The [Member](Self::BackendProvider).
    ///
    /// returns: [Result]<[Self], [EidError]> [Self] if creation of the EID succeeds, [EidError] otherwise
    ///
    fn create_eid(
        initial_member: &Self::MemberProvider,
        key_pair: Self::KeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create an [EidClient] with the state of an existing EID that the clients corresponding [Member](Self::MemberProvider) is invited to.
    ///
    /// # Arguments
    ///
    /// * `invitation`: The [Evolvement](Self::EvolvementProvider) that was created when the [Member](Self::MemberProvider) was added to the EID (see [Self::add])
    /// * `signature_keypair`: The invited [Member](Self::MemberProvider)'s key material
    /// * `backend`: The [EidBackend](Self::BackendProvider)
    ///
    /// returns: [Result]<[Self], [EidError]> [Self] if creation of the EID succeeds, [EidError] otherwise
    fn create_from_invitation(
        invitation: Self::EvolvementProvider,
        signature_keypair: Self::KeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement](Self::EvolvementProvider) that can be used to to add a [Member](Self::MemberProvider) to the EID.
    /// The evolvement is used to create the added members client from invitation (see [create_from_invitation](Self::create_from_invitation)).
    ///
    /// # Arguments
    ///
    /// * `member`: The [Member](Self::MemberProvider)
    /// * `backend`: The [EidBackend](Self::BackendProvider)
    ///
    /// returns: [Result]<[Evolvement](Self::EvolvementProvider), [EidError]> [Evolvement](Self::EvolvementProvider) if creation of the EID succeeds, [EidError] otherwise.
    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>;

    /// Create an [Evolvement](Self::EvolvementProvider) to remove a member from the EID.
    ///
    /// # Arguments
    ///
    /// * `member`: The [Member](Self::MemberProvider)
    /// * `backend`: The [Backend](Self::BackendProvider)
    ///
    /// returns: [Result]<[Self::EvolvementProvider], [EidError]>
    ///
    fn remove(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement](Self::EvolvementProvider) to update your own key material.
    ///
    /// # Arguments
    ///
    /// * `backend`: The [Backend](Self::BackendProvider)
    ///
    /// returns: [Result]<[Self::EvolvementProvider], [EidError]>
    ///
    fn update(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>;

    /// Apply the next [Evolvement](Self::EvolvementProvider), changing the client's state.
    ///
    /// # Arguments
    ///
    /// * `evolvement`: The [Evolvement](Self::EvolvementProvider)
    /// * `backend`: The [Backend](Self::BackendProvider)
    ///
    /// returns: [Result]<[()], [EidError]> [EidError] If the [Self::EvolvementProvider] is invalid.
    ///
    fn evolve(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Evolve the client's state by calling [evolve](Self::evolve)
    /// for each [Evolvement](Self::EvolvementProvider) in a [Vec] of [Evolvement](Self::EvolvementProvider)s.
    ///
    /// # Arguments
    ///
    /// * `evolvements`: The [Vec]<[Self::EvolvementProvider]>
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: [Result]<[()], [EidError]>) [EidError] If any [Self::EvolvementProvider] is invalid.
    ///
    fn batch_evolve(
        &mut self,
        evolvements: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        for evolvement in evolvements.iter() {
            self.evolve(evolvement.clone(), backend)?;
        }
        Ok(())
    }

    /// Cross-sign a clients membership.
    /// A member that was invited to a group via [add](Self::add) needs to cross-sign to be part of the EID.
    ///
    /// # Arguments
    ///
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: [Result]<[Self::EvolvementProvider], [EidError]>
    ///
    fn cross_sign_membership(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>;

    /// Get all members of the EID.
    ///
    /// returns: [Vec]<[Self::MemberProvider]>
    fn get_members(&self) -> Vec<Self::MemberProvider>;

    /// Export the clients EidState as [Self::ExportedTranscriptStateProvider].
    /// The exported state will be the trusted trusted EID state used to create a Transcript.
    ///
    /// # Arguments
    ///
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: [Result]<[Self::ExportedTranscriptStateProvider], [EidError]>
    ///
    fn export_transcript_state(
        &self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::ExportedTranscriptStateProvider, EidError>;

    /// Generate a [member](Self::MemberProvider) that can be added to an EID.
    ///
    /// # Arguments
    ///
    /// * `id`: The members [Vec]<[u8]> identifier
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: ([Self::MemberProvider], [Self::KeyProvider])
    ///
    #[cfg(feature = "test")]
    fn generate_member(
        id: Vec<u8>,
        backend: &Self::BackendProvider,
    ) -> (Self::MemberProvider, Self::KeyProvider);

    /// Generate an initial [client](Self).
    /// The client will be the start of a new EID.
    ///
    /// # Arguments
    ///
    /// * `id`: The members [Vec]<[u8]> identifier
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: [Self]
    ///
    #[cfg(feature = "test")]
    fn generate_initial_client(id: Vec<u8>, backend: &Self::BackendProvider) -> Self;
}
