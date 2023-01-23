use eid_traits::state::EidState;
use eid_traits::transcript::EidTranscript;
use eid_traits::types::EidError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::EidDummyMember;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
pub struct EidDummyTranscript {
    trusted_state: EidDummyState,
    current_state: EidDummyState,
    log: Vec<EidDummyEvolvement>,
}

impl EidTranscript for EidDummyTranscript {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type BackendProvider = EidDummyBackend;
    type StateProvider = EidDummyState;

    fn new(
        trusted_state: EidDummyState,
        log: Vec<EidDummyEvolvement>,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let mut current_state = trusted_state.clone();
        current_state.apply_log(log.clone(), backend)?;
        let transcript = EidDummyTranscript {
            trusted_state,
            log,
            current_state,
        };
        Ok(transcript)
    }

    fn add_evolvement(
        &mut self,
        evolvement: EidDummyEvolvement,
        _backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        self.log.push(evolvement.clone());
        match evolvement {
            EidDummyEvolvement::Update { members }
            | EidDummyEvolvement::Add { members }
            | EidDummyEvolvement::Remove { members } => self.current_state.members = members,
        }
        Ok(())
    }

    fn trusted_state(&self) -> EidDummyState {
        self.trusted_state.clone()
    }

    fn log(&self) -> Vec<EidDummyEvolvement> {
        self.log.clone()
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.current_state.members.clone()
    }
}
