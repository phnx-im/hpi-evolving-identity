use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;
use openmls::prelude::{MlsGroup, ProcessedMessage};
use openmls_rust_crypto::OpenMlsRustCrypto;

use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::state_trait::EidMlsState;

pub(crate) struct EidMlsClientState {
    pub(crate) group: MlsGroup,
    pub(crate) backend: &'static OpenMlsRustCrypto, //todo resolve static lifetime
}

impl Clone for EidMlsClientState {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl EidMlsState for EidMlsClientState {
    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        match message {
            ProcessedMessage::ApplicationMessage(_) | ProcessedMessage::ProposalMessage(_) => {
                Err(EidError::InvalidMessageError)
            }
            ProcessedMessage::StagedCommitMessage(staged_commit) => {
                self.group
                    .merge_staged_commit(*staged_commit)
                    .map_err(|_| EidError::ApplyCommitError)
            }
        }
    }
}

impl EidState<EidMlsEvolvement, EidMlsMember> for EidMlsClientState {
    fn apply(&mut self, evolvement: &EidMlsEvolvement) -> Result<(), EidError> {
        let parsed_message = self
            .group
            .parse_message(evolvement.message.clone(), self.backend)
            .map_err(|_| EidError::ParseMessageError)?;
        let verified_message = self
            .group
            .process_unverified_message(parsed_message, None, self.backend)
            .map_err(|_| EidError::UnverifiedMessageError)?;
        self.apply_processed_message(verified_message)?;

        Ok(())
    }

    fn get_members(&self) -> Result<Vec<EidMlsMember>, EidError> {
        let key_packages = self.group.members();
        let members: Vec<EidMlsMember> = key_packages
            .iter()
            .map(|kp| EidMlsMember::new((*kp).clone()))
            .collect();
        Ok(members)
    }

    fn apply_log(&mut self, evolvements: &[EidMlsEvolvement]) -> Result<(), EidError> {
        for evolvement in evolvements {
            self.apply(evolvement)?;
        }
        Ok(())
    }

    fn verify_client(&self, member: &EidMlsMember) -> Result<bool, EidError> {
        Ok(self.get_members()?.contains(member))
    }
}

impl Eq for EidMlsClientState {}

impl PartialEq<Self> for EidMlsClientState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
