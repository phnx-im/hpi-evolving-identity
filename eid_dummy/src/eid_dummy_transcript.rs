use eid_traits::evolvement::Evolvement;
use eid_traits::transcript::Transcript;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
pub struct EidDummyTranscript {
    trusted_state: EidDummyState,
    log: Vec<EidDummyEvolvement>
}

impl Transcript for EidDummyTranscript {
    type StateProvider = EidDummyState;
    type EvolvementProvider = EidDummyEvolvement;

    fn new(trusted_state: Self::StateProvider, log: Vec<Self::EvolvementProvider>) -> Self {
        EidDummyTranscript {trusted_state, log}
    }

    fn add_evolvement(&self, evolvement: EidDummyEvolvement) {

    }

    fn trusted_state(&self) -> Self::StateProvider {
        self.trusted_state.clone()
    }

    fn log(&self) -> Vec<Self::EvolvementProvider> {
        self.log.clone()
    }
}