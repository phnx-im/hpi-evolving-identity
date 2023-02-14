use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::evolvement::Evolvement;

use crate::eid_dummy_member::EidDummyMember;

#[derive(Debug, Clone, TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub enum EidDummyEvolvement {
    Add {
        members: Vec<EidDummyMember>,
        invited_id: Vec<u8>,
    },
    Update {
        members: Vec<EidDummyMember>,
    },
    Remove {
        members: Vec<EidDummyMember>,
    },
}

impl Default for EidDummyEvolvement {
    fn default() -> Self {
        Self::Add {
            members: vec![],
            invited_id: vec![],
        }
    }
}

impl Evolvement for EidDummyEvolvement {}
