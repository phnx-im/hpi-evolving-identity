use eid_traits::state::EidState;
use eid_traits::types::EidError;
use eid_traits::types::Member;
use openmls::framing::{ProcessedMessage, UnverifiedMessage};
use openmls::group::MlsGroup;
use openmls::prelude::{OpenMlsCrypto, OpenMlsCryptoProvider};
use openmls_rust_crypto::OpenMlsRustCrypto;

use crate::eid_mls_client::EidMlsClient;
use crate::eid_mls_evolvement::EidMlsEvolvement;

pub enum EidMlsState<'a> {
    Client {
        group: MlsGroup,
        backend: &'a OpenMlsRustCrypto,
    },
    Transcript {
        group: MlsGroup,
        backend: &'a OpenMlsRustCrypto,
    },
}

impl EidMlsState {
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
}

impl Clone for EidMlsState {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for EidMlsState {}

impl PartialEq<Self> for EidMlsState {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a> EidState<EidMlsEvolvement> for EidMlsState<'a> {
    fn apply_log(&mut self, log: &Vec<EidMlsEvolvement>) -> Result<(), EidError>
    where
        Self: Sized,
    {
        for evolvement in log.iter() {
            self.apply(evolvement)?;
        }
        Ok(())
    }

    fn apply(&mut self, evolvement: &EidMlsEvolvement) -> Result<(), EidError> {
        match self {
            EidMlsState::Client { group, backend } => {
                let parsed_message = group
                    .parse_message(evolvement.message.clone(), backend)
                    .map_err(|| EidError::ParseMessageError)?;
                let verified_message = group
                    .process_unverified_message(parsed_message, None, backend)
                    .map_err(|| EidError::UnverifiedMessageError)?;

                self.apply_processed_message(verified_message)?;
            }
            EidMlsState::Transcript { group, backend } => {
                todo!()
            }
        }
        Ok(())
    }

    fn verify_client(&self, client: &Member) -> Result<bool, EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        match self {
            EidMlsState::Client { group, backend } => {
                let key_packages = group.members();
                let public_keys = key_packages
                    .iter()
                    .map(|kp| kp.credential().signature_key().as_slice());
                let members = public_keys.map(|pk| Member::new(pk.to_vec())).collect();
                Ok(members)
            }
            EidMlsState::Transcript { group, backend } => Ok(vec![]),
        }
    }
}
