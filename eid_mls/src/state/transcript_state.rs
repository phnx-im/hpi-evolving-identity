use mls_assist::group::Group as AssistedGroup;
use openmls::framing::{MlsMessageIn, MlsMessageOut, ProcessedMessage};
use openmls::prelude::{LeafNode, MlsMessageInBody, Verifiable};

use eid_traits::state::EidState;
use eid_traits::transcript::EidExportedTranscriptState;
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

    fn verify_member(&self, _: &Self::MemberProvider) -> bool {
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

pub enum EidMlsExportedTranscriptState {
    IN {
        group_info: MlsMessageIn,
        leaf_node: LeafNode,
    },
    OUT {
        group_info: MlsMessageOut,
        leaf_node: LeafNode,
    },
}

impl EidExportedTranscriptState for EidMlsExportedTranscriptState {
    type TranscriptStateProvider = EidMlsTranscriptState;
    type BackendProvider = EidMlsBackend;

    fn into_transcript_state(
        self,
        backend: &EidMlsBackend,
    ) -> Result<Self::TranscriptStateProvider, EidError> {
        if let EidMlsExportedTranscriptState::IN {
            group_info: message_in,
            leaf_node,
        } = self
        {
            if let MlsMessageInBody::GroupInfo(verifiable_group_info) = message_in.extract() {
                let group_info = verifiable_group_info
                    .verify(
                        &backend.mls_backend,
                        // todo: should we take the key out of the leaf node or take a separate one as function argument?
                        leaf_node.signature_key(),
                        backend.ciphersuite.signature_algorithm(),
                    )
                    .map_err(|_| EidError::UnverifiedMessageError)?;

                let group = AssistedGroup::new(group_info, leaf_node.clone());

                Ok(EidMlsTranscriptState::new(group))
            } else {
                Err(EidError::ExportGroupInfoError)
            }
        } else {
            Err(EidError::InvalidMessageError)
        }
    }
}
