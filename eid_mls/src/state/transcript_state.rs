use super::state_trait::EidMlsState;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};
use openmls::group::MlsGroup;
use openmls_rust_crypto::OpenMlsRustCrypto;

/// Eid Mls Transcript State
pub(crate) struct EidMlsTranscriptState<'a> {
    group: MlsGroup,
    backend: &'a OpenMlsRustCrypto,
}

impl EidState<EidMlsEvolvement> for EidMlsTranscriptState {
    fn apply(&mut self, evolvement: EidMlsEvolvement) -> Result<(), EidError> {
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
