use openmls::prelude::Member as MlsMember;

use eid_traits::member::Member;

#[derive(Debug, PartialEq)]
pub struct EidMlsMember {
    pub(crate) member: MlsMember,
}

impl Member for EidMlsMember {
    type IdentityProvider = MlsMember;

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
