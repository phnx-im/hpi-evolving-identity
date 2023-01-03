use openmls::group::MlsGroup;
use openmls::prelude::ProcessedMessage;
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::state::EidState;
use eid_traits::types::EidError;
use eid_traits::types::Member;

use crate::eid_mls_evolvement::EidMlsEvolvement;

use super::state_trait::EidMlsState;

pub(crate) struct EidMlsClientState {
    group: MlsGroup,
    backend: &'static OpenMlsRustCrypto, //todo resolve static lifetime
}

impl EidMlsState for EidMlsClientState {
    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        match message {
            ProcessedMessage::ApplicationMessage(_) | ProcessedMessage::ProposalMessage(_) => {
                return Err(EidError::InvalidMessageError)
            }
            ProcessedMessage::StagedCommitMessage(staged_commit) => {
                self.group
                    .merge_staged_commit(*staged_commit)
                    .map_err(|_| EidError::ApplyCommitError)?;
                Ok(())
            }
        }
    }
}

impl EidState<EidMlsEvolvement> for EidMlsClientState {
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

    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        let key_packages = self.group.members();
        let public_keys = key_packages
            .iter()
            .map(|kp| kp.credential().signature_key().as_slice());
        let members = public_keys.map(|pk| Member::new(pk.to_vec())).collect();
        Ok(members)
    }

    fn apply_log(&mut self, evolvements: &[EidMlsEvolvement]) -> Result<(), EidError> {
        for evolvement in evolvements {
            self.apply(evolvement)?;
        }
        Ok(())
    }

    fn verify_client(&self, member: &eid_traits::types::Member) -> Result<bool, EidError> {
        Ok(self.get_members()?.contains(member))
    }
}

impl Eq for EidMlsClientState {}

impl PartialEq<Self> for EidMlsClientState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
