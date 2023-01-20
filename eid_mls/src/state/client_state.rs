use openmls::framing::MlsMessageInBody;
use openmls::framing::ProcessedMessageContent::StagedCommitMessage;
use openmls::prelude::{MlsGroup, ProcessedMessage, ProtocolMessage};

use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::state_trait::EidMlsState;

pub struct EidMlsClientState {
    pub(crate) group: MlsGroup,
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
            Err(EidError::InvalidMessageError)
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
        if let EidMlsEvolvement::IN { message: mls_in } = evolvement {
            let body = mls_in.extract();
            return match body {
                MlsMessageInBody::PrivateMessage(_) => Err(EidError::ProcessMessageError),
                MlsMessageInBody::Welcome(_)
                | MlsMessageInBody::GroupInfo(_)
                | MlsMessageInBody::KeyPackage(_) => {
                    todo!()
                }
                MlsMessageInBody::PublicMessage(msg) => {
                    let protocol_message = ProtocolMessage::PublicMessage(msg);
                    let processed_message = self
                        .group
                        .process_message(&backend.mls_backend, protocol_message)
                        .map_err(|_| EidError::ProcessMessageError)?;

                    self.apply_processed_message(processed_message, &backend)?;

                    Ok(())
                }
            };
        } else {
            Err(EidError::InvalidMessageError)
        }
    }

    fn verify_member(&self, member: &Self::MemberProvider) -> bool {
        self.get_members().contains(member)
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.members.clone()
    }
}
