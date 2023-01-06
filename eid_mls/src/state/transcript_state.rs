use openmls::group::MlsGroup;
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

use super::state_trait::EidMlsState;

/// Eid Mls Transcript State
pub(crate) struct EidMlsTranscriptState {
    group: MlsGroup,
}

impl EidState for EidMlsTranscriptState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;

    fn apply(
        &mut self,
        evolvement: &Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
        todo!()
    }

    fn verify_client(&self, _: &Self::MemberProvider) -> Result<bool, EidError> {
        todo!()
    }

    fn apply_log(
        &mut self,
        _: &[EidMlsEvolvement],
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
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
