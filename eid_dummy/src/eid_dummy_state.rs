use eid_traits::evolvement::Evolvement;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};

#[derive(Default, Clone)]
pub struct EidDummyState {
    pub(crate) members: Vec<Member>,
}

impl EidState<EidDummyEvolvement> for EidDummyState {
    fn from_log(evolvements: Vec<EidDummyEvolvement>) -> Result<Self, EidError> {
        let members = evolvements.last().unwrap().clone().members;

        Ok(EidDummyState { members })
    }
    fn apply(&mut self, evolvement: EidDummyEvolvement) -> Result<(), EidError> {
        self.members = evolvement.members;
        Ok(())
    }
    fn verify(&self) -> Result<bool, EidError> {
        Ok(true)
    }
    fn verify_client(&self, _: Member) -> Result<bool, EidError> {
        Ok(true)
    }
    fn get_members(&self) -> Result<Vec<Member>, EidError> {
        Ok(self.members.clone())
    }
}
