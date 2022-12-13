use super::state_trait::EidMlsState;
use crate::eid_mls_evolvement::EidMlsEvolvement;

pub(crate) struct EidMlsClientState<'a> {
    group: MlsGroup,
    backend: &'a OpenMlsRustCrypto,
}

impl EidState<EidMlsEvolvement> for EidMlsClientState {
    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        match message {
            ProcessedMessage::ApplicationMessage(_) | ProcessedMessage::ProposalMessage(_) => {
                return Err(EidError::InvalidMessageError)
            }
            ProcessedMessage::StagedCommitMessage(staged_commit) => {
                self.group
                    .merge_staged_commit(*staged_commit)
                    .map_err(|| EidError::ApplyCommitError)? // TODO
            }
        }
    }

    fn apply(&mut self, evolvement: &EidMlsEvolvement) -> Result<(), EidError> {
        let parsed_message = group
            .parse_message(evolvement.message.clone(), backend)
            .map_err(|| EidError::ParseMessageError)?;
        let verified_message = group
            .process_unverified_message(parsed_message, None, backend)
            .map_err(|| EidError::UnverifiedMessageError)?;
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
}

impl Clone for EidMlsClientState {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for EidMlsClientState {}

impl PartialEq<Self> for EidMlsClientState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl EidMlsState for EidMlsClientState {}
