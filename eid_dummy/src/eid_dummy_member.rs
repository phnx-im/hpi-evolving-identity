use eid_traits::member::Member;

#[derive(Debug, Clone, Eq)]
pub struct EidDummyMember {
    pk: Vec<u8>,
}

impl PartialEq for EidDummyMember {
    fn eq(&self, other: &Self) -> bool {
        self.pk.eq(&other.pk)
    }
}

impl Member for EidDummyMember {
    type PubkeyProvider = Vec<u8>;

    fn new(cred: Vec<u8>) -> Self {
        EidDummyMember { pk: cred }
    }

    fn get_pk(&self) -> Vec<u8> {
        self.pk.clone()
    }
}
