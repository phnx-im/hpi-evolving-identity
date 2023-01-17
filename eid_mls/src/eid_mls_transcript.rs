use eid_traits::transcript::EidTranscript;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::transcript_state::EidMlsTranscriptState;

pub struct EidMlsTranscript {
    trusted_state: EidMlsTranscriptState,
    current_state: EidMlsTranscriptState,
    log: Vec<EidMlsEvolvement>,
}

impl EidTranscript for EidMlsTranscript {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type BackendProvider = EidMlsBackend;
    type StateProvider = EidMlsTranscriptState;

    fn new(
        trusted_state: Self::StateProvider,
        log: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
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

impl EidMlsTranscript {
    fn apply_log(
        &mut self,
        mut log: Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        self.current_state.apply_log(log.clone(), backend)?;
        self.log.append(&mut log)?;
        Ok(())
    }
}
