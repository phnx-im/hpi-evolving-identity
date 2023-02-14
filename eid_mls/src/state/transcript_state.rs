use std::io::{Read, Write};

use openmls::framing::{MlsMessageIn, MlsMessageOut, ProcessedMessage};
use openmls::group::PublicGroup;
use openmls::prelude::{
    LeafNode, MlsMessageInBody, Node, ProcessedMessageContent, ProposalStore, ProtocolMessage,
    Verifiable,
};
use openmls::prelude_test::ContentType;
use serde;
use serde_json;
use tls_codec::{
    Deserialize, Error as TlsError, Serialize, Size, TlsDeserialize, TlsSerialize, TlsSize,
};

use eid_traits::state::EidState;
use eid_traits::transcript::EidExportedTranscriptState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::client_state::EidMlsClientState;

use super::state_trait::EidMlsState;

/// Eid Mls Transcript State
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EidMlsTranscriptState {
    pub(crate) group: PublicGroup<false>,
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
        if let EidMlsEvolvement::IN { message, .. } = evolvement {
            let body = message.extract();
            if let MlsMessageInBody::PublicMessage(msg) = body {
                let pub_msg = ProtocolMessage::PublicMessage(msg.clone());
                let processed_message =
                    self.group
                        .process_message(&backend.mls_backend, pub_msg)
                        .map_err(|e| EidError::ProcessMessageError(e.to_string()))?;
                match processed_message.into_content() {
                    ProcessedMessageContent::ApplicationMessage(_)
                    | ProcessedMessageContent::ProposalMessage(_)
                    | ProcessedMessageContent::ExternalJoinProposalMessage(_) => {
                        return Err(EidError::ProcessMessageError(
                            "Unexpected message type.".into(),
                        ))
                    }
                    ProcessedMessageContent::StagedCommitMessage(staged_commit) => {
                        // Merge the diff
                        self.group
                            .merge_commit(*staged_commit)
                            .map_err(|e| EidError::ApplyCommitError(e.to_string))?
                    }
                };
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
    pub(crate) fn new(group: PublicGroup<false>) -> Self {
        EidMlsTranscriptState { group }
    }

    pub(crate) fn clone_serde(&self) -> Result<Self, EidError> {
        let serialized =
            serde_json::to_string(self).map_err(|e| EidError::SerializationError(e.to_string()))?;
        let deserialized = serde_json::from_str(serialized.into())
            .map_err(|e| EidError::DeserializationError(e.to_string()))?;
        Ok(deserialized)
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

impl Size for EidMlsExportedTranscriptState {
    fn tls_serialized_len(&self) -> usize {
        match self {
            Self::OUT { group_info, nodes } | Self::IN { group_info, nodes } => {
                let len = group_info.tls_serialized_len();
                let nodes_len = nodes.iter().map(|node| node.tls_serialized_len()).sum();
                len + nodes_len
            }
        }
    }
}

impl Serialize for EidMlsExportedTranscriptState {
    fn tls_serialize<W: Write>(&self, writer: &mut W) -> Result<usize, TlsError> {
        if let Self::OUT { group_info, nodes } = self {
            let mut bytes_written = 0;
            let msg_ser = group_info.tls_serialize_detached()?;
            bytes_written += writer.write(msg_ser.as_slice())?;

            let welcome_ser = nodes.tls_serialize_detached()?;
            bytes_written += writer.write(welcome_ser.as_slice())?;

            Ok(bytes_written)
        } else {
            Err(TlsError::EncodingError(String::from(
                "Expected EidMlsExportedTranscriptState::OUT, got ::IN",
            )))
        }
    }
}

impl Deserialize for EidMlsExportedTranscriptState {
    fn tls_deserialize<R: Read>(bytes: &mut R) -> Result<Self, TlsError>
    where
        Self: Sized,
    {
        let group_info = MlsMessageIn::tls_deserialize(bytes)?;
        let nodes = Vec::<Option<Node>>::tls_deserialize(bytes)?;
        Ok(Self::IN { group_info, nodes })
    }
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
                // let group_info = verifiable_group_info
                //     .verify(
                //         &backend.mls_backend,
                //         // todo: should we take the key out of the leaf node or take a separate one as function argument?
                //         leaf_node.signature_key(),
                //         backend.ciphersuite.signature_algorithm(),
                //     )
                //     .map_err(|_| EidError::UnverifiedMessageError)?;

                let group = AssistedGroup::new();

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
