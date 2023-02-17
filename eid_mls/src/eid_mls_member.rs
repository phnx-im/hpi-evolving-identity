use openmls::prelude::{CredentialWithKey, KeyPackage, Member as MlsMember};

use eid_traits::member::Member;

#[derive(Debug, Clone)]
pub struct EidMlsMember {
    // TODO: do we need a constant identifier here?
    pub(crate) mls_member: Option<MlsMember>,
    pub(crate) key_package: Option<KeyPackage>,
    pub(crate) credential: CredentialWithKey,
}

impl PartialEq for EidMlsMember {
    fn eq(&self, other: &Self) -> bool {
        self.credential.signature_key == other.credential.signature_key
    }
}

impl Member for EidMlsMember {
    type CredentialProvider = (KeyPackage, CredentialWithKey);

    fn new((key_package, credential): Self::CredentialProvider) -> Self {
        Self {
            mls_member: None,
            key_package: Some(key_package),
            credential,
        }
    }

    #[cfg(feature = "test")]
    fn get_pk(&self) -> Vec<u8> {
        self.credential.signature_key.as_slice().to_vec()
    }
}

impl EidMlsMember {
    pub(crate) fn from_existing(mls_member: MlsMember) -> Self {
        let signature_key = mls_member.signature_key.clone().into();
        let credential = mls_member.credential.clone();
        Self {
            mls_member: Some(mls_member),
            key_package: None,
            credential: CredentialWithKey {
                credential,
                signature_key,
            },
        }
    }
}
