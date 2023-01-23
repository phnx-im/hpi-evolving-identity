use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::EidDummyMember;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EidDummyState {
    pub(crate) members: Vec<EidDummyMember>,
}

impl EidState for EidDummyState {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type BackendProvider = EidDummyBackend;
    fn apply_log(
        &mut self,
        evolvements: Vec<EidDummyEvolvement>,
        backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        if let Some(evolvement) = evolvements.last() {
            self.apply(evolvement.clone(), backend)?;
        }
        Ok(())
    }

    fn apply(
        &mut self,
        evolvement: EidDummyEvolvement,
        _backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        match &evolvement {
            EidDummyEvolvement::Update { members }
            | EidDummyEvolvement::Add { members }
            | EidDummyEvolvement::Remove { members } => {
                self.members = members.clone();
                Ok(())
            }
        }
    }
    fn verify_member(&self, _: &EidDummyMember) -> Result<bool, EidError> {
        Ok(true)
    }
    fn get_members(&self) -> Result<Vec<EidDummyMember>, EidError> {
        Ok(self.members.clone())
    }
}
