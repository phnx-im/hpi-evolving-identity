use crate::eid_dummy_member::EidDummyMember;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_dummy_evolvement::EidDummyEvolvement;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EidDummyState {
    pub(crate) members: Vec<EidDummyMember>,
}

impl EidState<EidDummyEvolvement> for EidDummyState {
    fn apply_log(&mut self, evolvements: &[EidDummyEvolvement]) -> Result<(), EidError> {
        let evolvement = evolvements.last().unwrap();
        self.apply(evolvement)
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
    fn verify_client(&self, _: &EidDummyMember) -> Result<bool, EidError> {
        Ok(true)
    }
    fn get_members(&self) -> Result<Vec<EidDummyMember>, EidError> {
        Ok(self.members.clone())
    }
}
