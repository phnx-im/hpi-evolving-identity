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
        let mut transcript = EidDummyTranscript {
            current_state: trusted_state.clone(),
            log: log.clone(),
            trusted_state,
        };
        transcript.batch_evolve(log, backend)?;
        Ok(transcript)
    }

    fn evolve(
        &mut self,
        evolvement: EidDummyEvolvement,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        self.current_state.apply(evolvement.clone(), backend)?;
        self.log.push(evolvement);
        Ok(())
    }

    fn log(&self) -> Vec<EidDummyEvolvement> {
        self.log.clone()
    }
    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.current_state.members.clone()
    }
    fn get_trusted_state(&self) -> Result<Self::StateProvider, EidError> {
        Ok(self.trusted_state.clone())
    }
}
