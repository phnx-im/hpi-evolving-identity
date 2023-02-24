use openmls::framing::MlsMessageInBody;
use openmls::framing::ProcessedMessageContent::StagedCommitMessage;
use openmls::prelude::{LeafNode, Member as MlsMember};
use openmls::prelude::{
    MlsGroup, Node, ProcessMessageError, ProcessedMessage, ProtocolMessage, StageCommitError,
};

use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;

/// # EidClientState
/// Implementation of [EidState] using [openmls] wrapping a [MlsGroup].
pub struct EidMlsClientState {
    pub(crate) group: MlsGroup,
}

impl EidMlsClientState {
    fn apply_processed_message(
        &mut self,
        message: ProcessedMessage,
        backend: &EidMlsBackend,
    ) -> Result<(), EidError> {
        if let StagedCommitMessage(staged_commit_ref) = message.into_content() {
            self.group
                .merge_staged_commit(&backend.mls_backend, *staged_commit_ref)
                .map_err(|e| EidError::InvalidEvolvementError(e.to_string()))?;
            Ok(())
        } else {
            Err(EidError::InvalidEvolvementError(
                "Expected ProcessedMessage::StagedCommitMessage, got a different variant of a processed message".into(),
            ))
        }
    }
}

impl EidState for EidMlsClientState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;

    fn apply(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        if let EidMlsEvolvement::IN {
            message: mls_in, ..
        } = evolvement
        {
            let body = mls_in.extract();
            if let MlsMessageInBody::PublicMessage(public_message) = body {
                let protocol_message = ProtocolMessage::PublicMessage(public_message);

                self.merge_or_apply_commit(protocol_message, backend)
            } else {
                Err(EidError::InvalidEvolvementError(
                    "Expected MlsMessageInBody::PublicMessage, got another variant".into(),
                ))
            }
        } else {
            Err(EidError::InvalidEvolvementError(String::from(
                "Expected EidMlsEvolvement::IN, got ::OUT",
            )))
        }
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        // get members out of group state -> members leaf node sources which are not a key package but a commit, will be valid members
        self.group
            .members()
            .filter(|member| self.has_member(member).unwrap_or(false))
            .map(|member| EidMlsMember::from_existing(member.clone()))
            .collect()
    }
}

impl EidMlsClientState {
    fn merge_or_apply_commit(
        &mut self,
        protocol_message: ProtocolMessage,
        backend: &EidMlsBackend,
    ) -> Result<(), EidError> {
        let processed_message_result = self
            .group
            .process_message(&backend.mls_backend, protocol_message);

        match processed_message_result {
            Ok(processed_message) => {
                self.apply_processed_message(processed_message, &backend)?;
                Ok(())
            }
            Err(process_message_error) => {
                if let ProcessMessageError::InvalidCommit(stage_commit_error) =
                    process_message_error
                {
                    // if the commit belongs to ourselves, we can just merge the pending commit.
                    if let StageCommitError::OwnCommit = stage_commit_error {
                        self.group
                            .merge_pending_commit(&backend.mls_backend)
                            .map_err(|e| EidError::InvalidEvolvementError(e.to_string()))?;
                        return Ok(());
                    }
                }

                Err(EidError::InvalidEvolvementError(
                    "Failed to process MLS message".into(),
                ))
            }
        }
    }

    fn get_leaf_nodes(&self) -> Vec<LeafNode> {
        self.group
            .export_ratchet_tree()
            .iter()
            .filter_map(|node| match node {
                Some(Node::LeafNode(leaf_node)) => Some(LeafNode::from(leaf_node.clone())),
                Some(Node::ParentNode(_)) | None => None,
            })
            .collect()
    }

    /// True if the member has cross-signed their addition to the group.
    ///
    /// # Arguments
    ///
    /// * `member`:
    ///
    /// returns: Result<bool, EidError>
    fn has_member(&self, member: &MlsMember) -> Result<bool, EidError> {
        let leaf_nodes = self.get_leaf_nodes();

        let leaf_node: &LeafNode =
            leaf_nodes
                .get(member.index.u32() as usize)
                .ok_or(EidError::InvalidMemberError(
                    "Member index doesn't have a matching node".into(),
                ))?;

        Ok(leaf_node.parent_hash().is_some())
    }
}
