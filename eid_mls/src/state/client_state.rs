use openmls::framing::MlsMessageIn;
use openmls::framing::ProcessedMessageContent::StagedCommitMessage;
use openmls::prelude::{MlsGroup, ProcessedMessage, ProtocolMessage};

use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::state_trait::EidMlsState;

pub struct EidMlsClientState {
    pub(crate) group: MlsGroup,
}

impl EidMlsState for EidMlsClientState {
    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        if let StagedCommitMessage(staged_commit) = message.content() {
            self.group.merge_staged_commit(**staged_commit);
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
            let processed_message = self
                .group
                .process_message(&backend.mls_backend, mls_in)
                .map_err(|_| EidError::ProcessMessageError)?;

            self.apply_processed_message(processed_message)?;

            Ok(())
        } else {
            Err(EidError::InvalidMessageError)
        }
    }

    fn verify_member(&self, member: &Self::MemberProvider) -> Result<bool, EidError> {
        Ok(self.get_members()?.contains(member))
    }

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
        let members: Vec<EidMlsMember> = self
            .group
            .members()
            .map(|m| EidMlsMember::new(m.clone()))
            .collect();
        Ok(members)
    }
}
