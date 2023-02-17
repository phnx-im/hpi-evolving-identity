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
                .map_err(|e| EidError::ApplyCommitError(e.to_string()))?;
            Ok(())
        } else {
            Err(EidError::InvalidMessageError(format!(
                // TODO
                "Expected StagedCommitMessage, got XXX",
            )))
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
                Err(EidError::ProcessMessageError(
                    "Expected MlsMessageInBody::PublicMessage, got another variant".into(),
                ))
            }
        } else {
            Err(EidError::InvalidMessageError(String::from(
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
                            .map_err(|e| EidError::ApplyCommitError(e.to_string()))?;
                        return Ok(());
                    }
                }

                Err(EidError::ProcessMessageError(
                    "Failed to process message".into(),
                ))
            }
        }
    }

    fn get_leaf_nodes(&self) -> Vec<LeafNode> {
        let tree = self
            .group
            .export_ratchet_tree()
            .iter()
            .map(|node| node.clone().unwrap())
            .collect::<Vec<Node>>();

        let leaf_nodes = tree
            .iter()
            .filter_map(|node| match node {
                Node::LeafNode(leaf_node) => Some(LeafNode::from(leaf_node.clone())),
                Node::ParentNode(_) => None,
            })
            .collect();

        leaf_nodes
    }

    /// Returns true if the member has cross-signed their addition to the group.
    ///
    /// # Arguments
    ///
    /// * `member`:
    ///
    /// returns: Result<bool, EidError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn has_member(&self, member: &MlsMember) -> Result<bool, EidError> {
        // Todo: It would be great if mls offers a get_member_by_index method
        let leaf_nodes = self.get_leaf_nodes();

        let index = member.index;

        let leaf_node: &LeafNode =
            leaf_nodes
                .get(index.u32() as usize)
                .ok_or(EidError::InvalidMemberError(
                    "Member index doesn't have a matching node".into(),
                ))?;
        //leaf_node.credential()
        Ok(leaf_node.parent_hash().is_some())
    }
}
