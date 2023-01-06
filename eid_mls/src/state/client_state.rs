use openmls::prelude::{MlsGroup, OpenMlsCryptoProvider, ProcessedMessage};
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::state_trait::EidMlsState;

pub(crate) struct EidMlsClientState {
    pub(crate) group: MlsGroup,
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
            ProcessedMessage::StagedCommitMessage(staged_commit) => self
                .group
                .merge_staged_commit(*staged_commit)
                .map_err(|_| EidError::ApplyCommitError),
        }
    }
}

impl EidState for EidMlsClientState {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;
    fn apply(
        &mut self,
        evolvement: &Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        let parsed_message = self
            .group
            .parse_message(evolvement.message.clone(), backend)
            .map_err(|_| EidError::ParseMessageError)?;
        let verified_message = self
            .group
            .process_unverified_message(parsed_message, None, backend)
            .map_err(|_| EidError::UnverifiedMessageError)?;
        self.apply_processed_message(verified_message)?;

        Ok(())
    }

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
        let key_packages = self.group.members();
        let members: Vec<EidMlsMember> = key_packages
            .iter()
            .map(|kp| EidMlsMember::new((*kp).clone()))
            .collect();
        Ok(members)
    }

    fn apply_log(
        &mut self,
        evolvements: &[Self::EvolvementProvider],
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        for evolvement in evolvements {
            self.apply(evolvement, backend)?;
        }
        Ok(())
    }

    fn verify_client(&self, member: &Self::MemberProvider) -> Result<bool, EidError> {
        Ok(self.get_members()?.contains(member))
    }
}

impl Eq for EidMlsClientState {}

impl PartialEq<Self> for EidMlsClientState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
