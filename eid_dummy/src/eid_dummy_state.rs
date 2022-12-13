use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};

use crate::eid_dummy_evolvement::EidDummyEvolvement;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EidDummyState {
    pub(crate) members: Vec<Member>,
}

impl EidState<EidDummyEvolvement> for EidDummyState {
    fn apply_log(&mut self, evolvements: &Vec<EidDummyEvolvement>) -> Result<(), EidError> {
        let evolvement = evolvements.last().unwrap();
        match &evolvement {
            EidDummyEvolvement::Update { members }
            | EidDummyEvolvement::Add { members }
            | EidDummyEvolvement::Remove { members } => {
                self.members = members.clone();
                Ok(())
            }
        }
    }
    fn apply(&mut self, evolvement: &EidDummyEvolvement) -> Result<(), EidError> {
        match &evolvement {
            EidDummyEvolvement::Update { members }
            | EidDummyEvolvement::Add { members }
            | EidDummyEvolvement::Remove { members } => {
                self.members = members.clone();
                Ok(())
            }
        }
    }
    fn verify_client(&self, _: &Member) -> Result<bool, EidError> {
        Ok(true)
    }
    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        Ok(self.members.clone())
    }
}
