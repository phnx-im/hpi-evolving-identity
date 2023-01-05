use eid_traits::member::Member;
use openmls::prelude::KeyPackage;

pub(crate) struct EidMlsMember {
    key_package: KeyPackage,
}

impl EidMlsMember {
    pub(crate) fn get_key_package(&self) -> KeyPackage {
        self.key_package.clone()
    }
}

impl PartialEq for EidMlsMember {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Member for EidMlsMember {
    type CredentialProvider = KeyPackage;

    fn new(key_package: KeyPackage) -> Self {
        Self {
            key_package
        }
    }

    fn get_pk(&self) -> Vec<u8> {
        todo!()
    }
}