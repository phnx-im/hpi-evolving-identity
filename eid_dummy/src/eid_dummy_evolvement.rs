use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::evolvement::Evolvement;

use crate::eid_dummy_member::EidDummyMember;

#[derive(Debug, Clone, TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub enum EidDummyEvolvement {
    Add { members: Vec<EidDummyMember> },
    Update { members: Vec<EidDummyMember> },
    Remove { members: Vec<EidDummyMember> },
}

impl Default for EidDummyEvolvement {
    fn default() -> Self {
        Self::Add { members: vec![] }
    }
}

impl Evolvement for EidDummyEvolvement {
    fn is_valid_successor(&self, _previous: &Self) -> bool {
        true
    }
}
