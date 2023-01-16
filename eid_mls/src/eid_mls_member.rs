use openmls::prelude::Member as MlsMember;

use eid_traits::member::Member;

#[derive(Debug, PartialEq)]
pub struct EidMlsMember {
    pub(crate) member: MlsMember,
}

impl Member for EidMlsMember {
    type IdentityProvider = MlsMember;

    fn new(member: MlsMember) -> Self {
        Self { member }
    }

    fn get_identity(&self) -> Self::IdentityProvider {
        self.member.clone()
    }
}
