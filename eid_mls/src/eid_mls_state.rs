use openmls::framing::UnverifiedMessage;
use openmls::group::MlsGroup;
use openmls::prelude::{OpenMlsCrypto, OpenMlsCryptoProvider};
use crate::eid_mls_evolvement::EidMlsEvolvement;
use eid_traits::state::EidState;
use eid_traits::types::EidError
use eid_traits::types::Member;


pub enum EidMlsState {
    Client { group: MlsGroup },
    Transcript { group: MlsGroup}
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

impl EidState<EidMlsEvolvement> for EidMlsState {
    fn from_log(log: &Vec<EidMlsEvolvement>) -> Result<Self, EidError>
        where
            Self: Sized {
        todo!()
    }

    fn apply(&mut self, evolvement: &EidMlsEvolvement) -> Result<(), EidError> {
        match self {
            EidMlsState::Client{group} => {
                let parsed_message = group.parse_message(evolvement.message).m?;
                let verified_message = group.process_unverified_message(parsed_message)
            }
            EidMlsState::Transcript{ group} => {
                todo!()
            }
        }
        Ok(())
    }

    fn verify(&self) -> Result<bool, EidError> {
        todo!()
    }

    fn verify_client(&self, client: &Member) -> Result<bool, EidError> {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        todo!()
    }
}
