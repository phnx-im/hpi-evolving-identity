use openmls::framing::MlsMessageInBody;
use openmls::framing::ProcessedMessageContent::StagedCommitMessage;
use openmls::prelude::{
    MlsGroup, ProcessMessageError, ProcessedMessage, ProtocolMessage, StageCommitError,
};

use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::state_trait::EidMlsState;

pub struct EidMlsClientState {
    pub(crate) group: MlsGroup,
    // we have to maintain our own members list, because mls doesn't have a notion of identity
    pub(crate) members: Vec<EidMlsMember>,
}

impl EidMlsState for EidMlsClientState {
    fn apply_processed_message(
        &mut self,
        message: ProcessedMessage,
        backend: &EidMlsBackend,
    ) -> Result<(), EidError> {
        if let StagedCommitMessage(staged_commit_ref) = message.into_content() {
            self.group
                .merge_staged_commit(&backend.mls_backend, *staged_commit_ref)
                .map_err(|_| EidError::ApplyCommitError)?;
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
    fn apply_log(
        &mut self,
        evolvements: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        for evolvement in evolvements {
            self.apply(evolvement, backend)?;
        }
        Ok(())
    }

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

    fn verify_member(&self, member: &Self::MemberProvider) -> bool {
        self.get_members().contains(member)
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.members.clone()
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
                            .map_err(|e| EidError::ApplyCommitError)?;
                        return Ok(());
                    }
                }

                Err(EidError::ProcessMessageError(
                    "Failed to process message".into(),
                ))
            }
        }
    }
}
