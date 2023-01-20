use openmls::prelude::{KeyPackage, Member as MlsMember};

use eid_traits::member::Member;

#[derive(Debug, PartialEq, Clone)]
pub struct EidMlsMember {
    // TODO: do we need a constant identifier here?
    pub(crate) mls_member: Option<MlsMember>,
    pub(crate) key_package: KeyPackage,
}

impl Member for EidMlsMember {
    type IdentityProvider = KeyPackage;

    fn new(key_package: Self::IdentityProvider) -> Self {
        Self {
            mls_member: None,
            key_package,
        }
    }

    #[cfg(feature = "test")]
    fn get_identity(&self) -> Self::IdentityProvider {
        self.key_package.clone()
    }
}

impl EidMlsMember {
    fn set_member(&mut self, mls_member: MlsMember) {
        self.mls_member = Some(mls_member);
    }

    fn update_key_package(&mut self, new_key_package: <Self as Member>::IdentityProvider) {
        self.key_package = new_key_package;
    }
}
