use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};
use crate::eid_dummy_evolvement::EidDummyEvolvement;

#[derive(Default, Clone)]
pub struct EidDummyState {
    members: Vec<Member>,
}

impl EidState for EidDummyState {

    type EvolvementProvider = EidDummyEvolvement;

    fn from_log(evolvements: Vec<EidDummyEvolvement>) -> Result<Self, EidError> {
        let members = evolvements.last()
            .unwrap()
            .clone()
            .members;

        Ok(EidDummyState{
            members
        })
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
    fn get_clients(&self) -> Result<Vec<Member>, EidError> {
        Ok(self.members.clone())
    }
}