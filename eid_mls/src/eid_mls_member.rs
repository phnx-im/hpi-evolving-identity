use openmls::prelude::KeyPackage;
use openmls::prelude::Member as MlsMember;

use eid_traits::member::Member;

#[derive(Debug, PartialEq)]
pub struct EidMlsMember {
    pub(crate) member: MlsMember,
}

impl PartialEq for EidMlsMember {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Member for EidMlsMember {
    type PubkeyProvider = KeyPackage;

    fn new(key_package: KeyPackage) -> Self {
        Self { key_package }
    }

    fn get_pk(&self) -> Self::PubkeyProvider {
        self.key_package.clone()
    }
}
