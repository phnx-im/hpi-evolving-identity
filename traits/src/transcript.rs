use tls_codec::{Deserialize, Serialize};

use crate::backend::EidBackend;
use crate::evolvement::Evolvement;
use crate::member::Member;
use crate::state::EidState;
use crate::types::EidError;

pub trait EidTranscript {
    type EvolvementProvider: Evolvement;
    type MemberProvider: Member;
    type BackendProvider: EidBackend;
    type StateProvider: EidState<
        EvolvementProvider = Self::EvolvementProvider,
        MemberProvider = Self::MemberProvider,
    >;
    /// creates a new log from a trusted [EidState] and a vector of evolvements that happened after the trusted [EidState].
    fn new(
        trusted_state: Self::StateProvider,
        log: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Add a new entry on top of the existing [Evolvement]s in the transcript.
    fn evolve(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>;

    /// Return the [Evolvement]s that happened after the trusted [EidState].
    fn log(&self) -> Vec<Self::EvolvementProvider>;

    ///Return the current members (i.e, after the latest [Evolvement])
    fn get_members(&self) -> Vec<Self::MemberProvider>;

    fn get_trusted_state(&self) -> Result<Self::StateProvider, EidError>;
}

/// State that is exported by the client and sent over the wire. The only function it needs
/// to implement is the conversion to a transcript state.
pub trait EidExportedTranscriptState: Serialize + Deserialize {
    type TranscriptStateProvider: EidState;
    type BackendProvider: EidBackend;

    fn into_transcript_state(
        self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::TranscriptStateProvider, EidError>;
}
