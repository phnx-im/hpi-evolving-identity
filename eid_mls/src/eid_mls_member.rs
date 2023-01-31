use openmls::prelude::{CredentialWithKey, KeyPackage, Member as MlsMember, SignaturePublicKey};
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use eid_traits::member::Member;

#[derive(Debug, Clone)]
pub struct EidMlsMember {
    // TODO: do we need a constant identifier here?
    pub(crate) mls_member: Option<MlsMember>,
    pub(crate) key_package: Option<KeyPackage>,
    pub(crate) signature_key: SignaturePublicKey,
    pub(crate) credential: CredentialWithKey,
}

impl PartialEq for EidMlsMember {
    fn eq(&self, other: &Self) -> bool {
        self.signature_key == other.signature_key
    }
}

impl Member for EidMlsMember {
    type CredentialProvider = KeyPackage;

    fn new(key_package: Self::CredentialProvider) -> Self {
        let kp_signature_key = key_package.leaf_node().signature_key();
        Self {
            mls_member: None,
            key_package: Some(key_package),
            signature_key: kp_signature_key.clone(),
        }
    }

    #[cfg(feature = "test")]
    fn get_credential(&self) -> Self::CredentialProvider {
        self.key_package
            .clone()
            .expect("Doesn't contain key package")
    }
}

impl EidMlsMember {
    fn set_member(&mut self, mls_member: MlsMember) {
        self.mls_member = Some(mls_member);
    }

    fn update_signature_key(&mut self, new_signature_key: SignaturePublicKey) {
        self.signature_key = new_signature_key;
    }

    pub(crate) fn from_existing(
        mls_member: Option<MlsMember>,
        signature_key: SignaturePublicKey,
    ) -> Self {
        Self {
            mls_member,
            key_package: None,
            signature_key,
        }
    }
}
