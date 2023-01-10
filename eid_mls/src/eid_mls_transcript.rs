use eid_traits::transcript::EidTranscript;

use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::transcript_state::EidMlsTranscriptState;

pub struct EidMlsTranscript {}

impl Default for EidMlsTranscript {
    fn default() -> Self {
        Self {}
    }
}

impl EidTranscript for EidMlsTranscript {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type StateProvider = EidMlsTranscriptState;

    fn new(trusted_state: Self::StateProvider, log: Vec<Self::EvolvementProvider>) -> Self {
        todo!()
    }

    fn add_evolvement(&mut self, evolvement: Self::EvolvementProvider) {
        todo!()
    }

    fn trusted_state(&self) -> Self::StateProvider {
        todo!()
    }

    fn log(&self) -> Vec<Self::EvolvementProvider> {
        todo!()
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        todo!()
    }
}
