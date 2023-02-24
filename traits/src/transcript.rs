use tls_codec::{Deserialize, Serialize};

use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::types::EidError;

/// # EidTranscript
/// The Public Transcript of an EID. The Transcript holds a trusted [EidState] and a log of [Evolvement]s.
/// It calculates the current [EidState] by applying [Evolvement]s like clients do. It knows all [Member]s that are in the EID.
/// It cannot create any [Evolvement].
pub trait EidTranscript {
    /// Type of [Evolvement](Self::EvolvementProvider) a client creates.
    type EvolvementProvider: Evolvement;

    /// Type of [Member](Self::MemberProvider)s that can be added or removed from the EID.
    type MemberProvider: Member;

    /// Type of [EidBackend](Self::BackendProvider) this [EidTranscript](Self) uses.
    type BackendProvider: EidBackend;

    /// Type of [EidState](Self::StateProvider) this [EidTranscript](Self) uses
    type StateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;
    /// Creates a new log from a trusted [EidState] and a [Vec] of [Evolvement]s
    /// that happened after the trusted [EidState].
    ///
    /// # Arguments
    ///
    /// * `trusted_state`:
    /// * `log`:
    /// * `backend`:
    ///
    /// returns: [Result]<[Self], [EidError]> [EidError] if trusted_state or log is invalid.
    ///
    fn new(
        trusted_state: Self::StateProvider,
        log: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Apply the next [Evolvement](Self::EvolvementProvider), changing the transcript's current state and appending the [Evolvement](Self::EvolvementProvider) to the log.
    ///
    /// # Arguments
    ///
    /// * `evolvement`:
    /// * `backend`:
    ///
    /// returns: Result<(), EidError>
    ///
    fn evolve(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Evolve the transcript's current state by calling [evolve](Self::evolve)
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
    /// Get the [Evolvement]s that happened after the trusted [EidState].
    ///
    /// returns: [Vec]<[Self::EvolvementProvider]>
    fn log(&self) -> Vec<Self::EvolvementProvider>;

    /// Get all [Member]s of the EID.
    ///
    /// returns: [Vec]<[Self::MemberProvider]>
    fn get_members(&self) -> Vec<Self::MemberProvider>;

    /// Get the trusted [EidState] that is saved to the [EidTranscript](Self)
    fn get_trusted_state(&self) -> Result<Self::StateProvider, EidError>;
}

/// State that is exported by the client and sent over the wire. The only function it needs
/// to implement is the conversion to a transcript state.
pub trait EidExportedTranscriptState: Serialize + Deserialize {
    /// Type of [EidState] that the [EidTranscript] uses.
    type TranscriptStateProvider: EidState;

    /// Type of [EidBackend](Self::BackendProvider) this [EidExportedTranscriptState](Self) uses.
    type BackendProvider: EidBackend;

    /// Get a [EidState] from this [EidExportedTranscriptState](Self).
    ///
    /// # Arguments
    ///
    /// * `backend`: The [Self::BackendProvider]
    ///
    /// returns: [Result]<[Self]::[TranscriptStateProvider], [EidError]>
    ///
    fn into_transcript_state(
        self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::TranscriptStateProvider, EidError>;
}
