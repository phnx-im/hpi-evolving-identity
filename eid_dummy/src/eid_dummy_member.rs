use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::member::Member;

#[derive(Debug, Clone, Eq, TlsSerialize, TlsDeserialize, TlsSize)]
pub struct EidDummyMember {
    pub(crate) pk: Vec<u8>,
    pub(crate) cross_signed: BOOLEAN,
}

#[derive(Debug, Clone, Eq, PartialEq, TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub enum BOOLEAN {
    TRUE = 1,
    FALSE = 0,
}

impl PartialEq for EidDummyMember {
    fn eq(&self, other: &Self) -> bool {
        self.pk.eq(&other.pk)
    }
}

impl Member for EidDummyMember {
    type CredentialProvider = Vec<u8>;

    fn new(cred: Vec<u8>) -> Self {
        EidDummyMember {
            pk: cred,
            cross_signed: BOOLEAN::FALSE,
        }
    }

    #[cfg(feature = "test")]
    fn get_credential(&self) -> Vec<u8> {
        self.pk.clone()
    }
}
