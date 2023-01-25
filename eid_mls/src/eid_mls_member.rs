use openmls::prelude::{KeyPackage, Member as MlsMember};
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::member::Member;

#[derive(Debug, PartialEq, Clone)]
pub struct EidMlsMember {
    // TODO: do we need a constant identifier here?
    pub(crate) mls_member: Option<MlsMember>,
    pub(crate) key_package: Option<KeyPackage>,
    pub(crate) signature_key: Vec<u8>,
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
            .expect("Doesn't contain key package")
            .clone()
    }
}

impl EidMlsMember {
    fn set_member(&mut self, mls_member: MlsMember) {
        self.mls_member = Some(mls_member);
    }

    fn update_signature_key(&mut self, new_signature_key: Vec<u8>) {
        self.signature_key = new_signature_key;
    }
}
