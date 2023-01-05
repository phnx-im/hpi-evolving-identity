use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;
use openmls::group::MlsGroup;
use openmls_rust_crypto::OpenMlsRustCrypto;

use crate::eid_mls_evolvement::EidMlsEvolvement;

use super::state_trait::EidMlsState;

/// Eid Mls Transcript State
pub(crate) struct EidMlsTranscriptState {
    group: MlsGroup,
    backend: &'static OpenMlsRustCrypto,
}

impl<M: Member> EidState<EidMlsEvolvement, M> for EidMlsTranscriptState {
    fn apply(&mut self, evolvement: &EidMlsEvolvement) -> Result<(), EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<M>, EidError> {
        todo!()
    }

    fn verify_client(&self, _: &M) -> Result<bool, EidError> {
        todo!()
    }

    fn apply_log(&mut self, _: &[EidMlsEvolvement]) -> Result<(), EidError> {
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
