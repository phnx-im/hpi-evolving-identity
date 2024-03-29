use std::io::{Read, Write};

use openmls::group::PublicGroup;
use openmls::prelude::{
    LeafNode, Member as MlsMember, MlsMessageIn, MlsMessageInBody, MlsMessageOut, Node,
    ProcessedMessageContent, ProposalStore, ProtocolMessage,
};
use serde;
use serde_json;
use tls_codec::{Deserialize, Error as TlsError, Serialize, Size};

use eid_traits::state::EidState;
use eid_traits::transcript::EidExportedTranscriptState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

/// # EidTranscriptState
/// Implementation of [EidState] for the Transcript using [openmls] wrapping a [PublicGroup].
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EidMlsTranscriptState {
    pub(crate) group: PublicGroup,
}

impl EidState for EidMlsTranscriptState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;

    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        if let EidMlsEvolvement::IN { message, .. } = evolvement {
            let body = message.extract();
            if let MlsMessageInBody::PublicMessage(public_message) = body {
                let protocol_message = ProtocolMessage::PublicMessage(public_message);
                let processed_message = self
                    .group
                    .process_message(&backend.mls_backend, protocol_message)
                    .map_err(|e| EidError::InvalidEvolvementError(e.to_string()))?;
                match processed_message.into_content() {
                    ProcessedMessageContent::ApplicationMessage(_)
                    | ProcessedMessageContent::ProposalMessage(_)
                    | ProcessedMessageContent::ExternalJoinProposalMessage(_) => {
                        return Err(EidError::InvalidEvolvementError(
                            "Unexpected message type.".into(),
                        ))
                    }
                    ProcessedMessageContent::StagedCommitMessage(staged_commit) => {
                        self.group.merge_commit(*staged_commit);
                    }
                };
                Ok(())
            } else {
                Err(EidError::InvalidEvolvementError(format!(
                    "Expected PublicMessage, got {:?}",
                    body
                )))
            }
        } else {
            Err(EidError::InvalidEvolvementError(String::from(
                "Expected EidMlsEvolvement::IN, got ::OUT",
            )))
        }
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.group
            .members()
            .filter(|member| self.has_member(member).unwrap_or(false))
            .map(EidMlsMember::from_existing)
            .collect()
    }
}

impl EidMlsTranscriptState {
    fn has_member(&self, member: &MlsMember) -> Result<bool, EidError> {
        let leaf_nodes = self.get_leaf_nodes();
        let leaf_node: &LeafNode = leaf_nodes
            .iter()
            .find(|&node| node.signature_key().as_slice().to_vec() == member.signature_key)
            .ok_or_else(|| {
                EidError::InvalidMemberError("Member index doesn't have a matching node".into())
            })?;
        Ok(leaf_node.parent_hash().is_some())
    }

    fn get_leaf_nodes(&self) -> Vec<LeafNode> {
        self.group
            .export_nodes()
            .iter()
            .filter_map(|node| match node {
                Some(Node::LeafNode(leaf_node)) => Some(LeafNode::from(leaf_node.clone())),
                Some(Node::ParentNode(_)) | None => None,
            })
            .collect()
    }
}

impl EidMlsTranscriptState {
    pub(crate) fn new(group: PublicGroup) -> Self {
        EidMlsTranscriptState { group }
    }

    pub(crate) fn clone_serde(&self) -> Result<Self, EidError> {
        let serialized =
            serde_json::to_string(self).map_err(|e| EidError::SerializationError(e.to_string()))?;
        let deserialized = serde_json::from_str(&serialized)
            .map_err(|e| EidError::DeserializationError(e.to_string()))?;
        Ok(deserialized)
    }
}

/// # EID MLS Exported Transcript State
/// Implementation of [EidExportedTranscriptState] using [openmls].
pub enum EidMlsExportedTranscriptState {
    IN {
        group_info: MlsMessageIn,
        nodes: Vec<Option<Node>>,
    },
    OUT {
        group_info: MlsMessageOut,
        nodes: Vec<Option<Node>>,
    },
}

impl Size for EidMlsExportedTranscriptState {
    fn tls_serialized_len(&self) -> usize {
        let nodes_len: usize = match self {
            EidMlsExportedTranscriptState::IN { nodes, .. }
            | EidMlsExportedTranscriptState::OUT { nodes, .. } => {
                nodes.iter().map(|node| node.tls_serialized_len()).sum()
            }
        };
        match self {
            Self::OUT { group_info, .. } => nodes_len + group_info.tls_serialized_len(),
            Self::IN { group_info, .. } => nodes_len + group_info.tls_serialized_len(),
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
            nodes,
        } = self
        {
            if let MlsMessageInBody::GroupInfo(verifiable_group_info) = message_in.extract() {
                let (group, _extensions) = PublicGroup::from_external(
                    &backend.mls_backend,
                    nodes.to_vec(),
                    verifiable_group_info,
                    ProposalStore::new(),
                )
                .map_err(|e| EidError::CreateTranscriptStateError(e.to_string()))?;

                Ok(EidMlsTranscriptState::new(group))
            } else {
                Err(EidError::ImportTranscriptStateError(
                    "Expected MlsMessageInBody::GroupInfo, got another variant of MlsMessageInBody"
                        .into(),
                ))
            }
        } else {
            Err(EidError::ImportTranscriptStateError(String::from(
                "Expected EidMlsExportedTranscriptState::IN, got ::OUT",
            )))
        }
    }
}
