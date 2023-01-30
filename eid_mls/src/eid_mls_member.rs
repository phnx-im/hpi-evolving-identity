use openmls::prelude::{KeyPackage, Member as MlsMember};
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::member::Member;

#[derive(Debug, Clone)]
pub struct EidMlsMember {
    // TODO: do we need a constant identifier here?
    pub(crate) mls_member: Option<MlsMember>,
    pub(crate) key_package: Option<KeyPackage>,
    pub(crate) signature_key: Vec<u8>,
}

impl PartialEq for EidMlsMember {
    fn eq(&self, other: &Self) -> bool {
        self.signature_key == other.signature_key
    }
}

impl Member for EidMlsMember {
    type IdentityProvider = KeyPackage;

    fn new(key_package: Self::IdentityProvider) -> Self {
        let kp_signature_key = key_package.leaf_node().signature_key().as_slice().to_vec();
        Self {
            mls_member: None,
            key_package: Some(key_package),
            signature_key: kp_signature_key,
        }
    }

    #[cfg(feature = "test")]
    fn get_identity(&self) -> Self::IdentityProvider {
        self.key_package
            .clone()
            .expect("Doesn't contain key package")
    }
}

impl EidMlsMember {
    fn set_member(&mut self, mls_member: MlsMember) {
        self.mls_member = Some(mls_member);
    }

    fn update_signature_key(&mut self, new_signature_key: Vec<u8>) {
        self.signature_key = new_signature_key;
    }

    pub(crate) fn from_existing(mls_member: Option<MlsMember>, signature_key: Vec<u8>) -> Self {
        Self {
            mls_member,
            key_package: None,
            signature_key,
        }
    }
}
