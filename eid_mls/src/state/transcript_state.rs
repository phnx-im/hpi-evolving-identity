use super::state_trait::EidMlsState;
use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};

/// Eid Mls Transcript State
pub(crate) struct EidMlsTranscriptState<'a> {
    group: MlsGroup,
    backend: &'a OpenMlsRustCrypto,
}

impl EidState for EidMlsTranscriptState {
    fn apply(&mut self, evolvement: &T) -> Result<(), EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        todo!()
    }
}

impl Clone for EidMlsTranscriptState {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for EidMlsTranscriptState {}

impl PartialEq<Self> for EidMlsTranscriptState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl EidMlsState for EidMlsTranscriptState {}
