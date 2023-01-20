use mls_assist::group::Group as AssistedGroup;
use openmls::framing::ProcessedMessage;

use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

use super::state_trait::EidMlsState;

/// Eid Mls Transcript State
pub struct EidMlsTranscriptState {
    pub(crate) group: AssistedGroup,
}

impl EidState for EidMlsTranscriptState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;

    fn apply_log(
        &mut self,
        _: Vec<EidMlsEvolvement>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        todo!()
    }

    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        todo!()
    }

    fn verify_member(&self, _: &Self::MemberProvider) -> Result<bool, EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
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

impl EidMlsState for EidMlsTranscriptState {
    fn apply_processed_message(
        &mut self,
        message: ProcessedMessage,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        todo!()
    }
}

impl EidMlsTranscriptState {
    pub(crate) fn new(group: AssistedGroup) -> Self {
        EidMlsTranscriptState { group }
    }
}
