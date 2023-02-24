use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::state::EidState;
use eid_traits::transcript::EidExportedTranscriptState;
use eid_traits::types::EidError;
use eid_traits::types::EidError::InvalidEvolvementError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::EidDummyMember;

/// # EID Dummy State
/// Dummy implementation of [EidState].
/// Holds [EidDummyMember]s and a count of processed [Evolvement]s for a minimum of validation
/// to provide compliance with integration tests.
#[derive(Default, Debug, Clone, PartialEq, Eq, TlsDeserialize, TlsSerialize, TlsSize)]
pub struct EidDummyState {
    pub(crate) members: Vec<EidDummyMember>,
    pub(crate) evolvement_count: u64,
}

impl EidState for EidDummyState {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type BackendProvider = EidDummyBackend;

    fn apply(
        &mut self,
        evolvement: EidDummyEvolvement,
        _backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        match &evolvement {
            EidDummyEvolvement::Update { members, count, .. }
            | EidDummyEvolvement::Add { members, count, .. }
            | EidDummyEvolvement::Remove { members, count, .. } => {
                if self.evolvement_count + 1 != *count {
                    return Err(InvalidEvolvementError("Invalid Evolvement count".into()));
                }
                self.evolvement_count += 1;
                self.members = members.clone();
                Ok(())
            }
        }
    }

    fn get_members(&self) -> Vec<EidDummyMember> {
        self.members.clone()
    }
}

impl EidExportedTranscriptState for EidDummyState {
    type TranscriptStateProvider = EidDummyState;
    type BackendProvider = EidDummyBackend;

    fn into_transcript_state(self, _backend: &EidDummyBackend) -> Result<EidDummyState, EidError> {
        Ok(self.clone())
    }
}
