use mls_assist::group::Group as AssistedGroup;
use mls_assist::messages::assisted_messages::{AssistedCommit, AssistedGroupInfo, AssistedMessage};
use openmls::framing::{MlsMessageIn, MlsMessageOut, ProcessedMessage};
use openmls::prelude::{LeafNode, MlsMessageInBody, ProtocolMessage, Verifiable};
use openmls::prelude_test::ContentType;
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::state::EidState;
use eid_traits::transcript::EidExportedTranscriptState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

use super::state_trait::EidMlsState;

/// Eid Mls Transcript State
#[derive(Clone)]
pub struct EidMlsTranscriptState {
    pub(crate) group: AssistedGroup,
}

impl EidState for EidMlsTranscriptState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;

    fn apply_log(
        &mut self,
        log: Vec<EidMlsEvolvement>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        for evolvement in log {
            self.apply(evolvement, backend)?;
        }
        Ok(())
    }

    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        if let EidMlsEvolvement::IN { message } = evolvement {
            let body = message.extract();
            if let MlsMessageInBody::PublicMessage(msg) = body {
                let pub_msg = ProtocolMessage::PublicMessage(msg.clone());

                let a_msg = match pub_msg.content_type() {
                    ContentType::Application | ContentType::Proposal => {
                        AssistedMessage::NonCommit(msg)
                    }
                    ContentType::Commit => {
                        // TODO: How do we get the signature from a commit?
                        let a_group_info = AssistedGroupInfo::Signature(Vec::new());
                        let a_commit = AssistedCommit::new(msg, a_group_info);
                        AssistedMessage::Commit(a_commit)
                    }
                };
                // TODO: I guess we want to do something with this :D
                let _processed_msg = self.group.process_message(a_msg);

                Ok(())
            } else {
                Err(EidError::InvalidMessageError(format!(
                    "Expected PublicMessage, got {:?}",
                    body
                )))
            }
        } else {
            Err(EidError::InvalidMessageError(String::from(
                "Expected EidMlsEvolvement::IN, got ::OUT",
            )))
        }
    }

    fn verify_member(&self, _: &Self::MemberProvider) -> bool {
        todo!()
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
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

#[derive(TlsSize, TlsDeserialize, TlsSerialize)]
#[repr(u8)]
pub enum EidMlsExportedTranscriptState {
    #[tls_codec(discriminant = 1)]
    IN {
        group_info: MlsMessageIn,
        leaf_node: LeafNode,
    },
    #[tls_codec(discriminant = 1)]
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
            Err(EidError::InvalidMessageError(String::from(
                "Expected EidMlsExportedTranscriptState::IN, got ::OUT",
            )))
        }
    }
}
