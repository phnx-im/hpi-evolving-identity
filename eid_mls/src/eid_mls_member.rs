use eid_traits::member::Member;
use openmls::prelude::KeyPackage;

pub struct EidMlsMember {
    pub(crate) key_package: KeyPackage,
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
