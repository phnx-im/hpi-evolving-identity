use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::evolvement::Evolvement;

use crate::eid_dummy_member::EidDummyMember;

#[derive(Debug, Clone, TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub enum EidDummyEvolvement {
    Add {
        count: u64,
        members: Vec<EidDummyMember>,
        invited_id: Vec<u8>,
    },
    Update {
        count: u64,
        members: Vec<EidDummyMember>,
    },
    Remove {
        count: u64,
        members: Vec<EidDummyMember>,
    },
}

impl Evolvement for EidDummyEvolvement {}
