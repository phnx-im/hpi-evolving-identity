use eid_traits::evolvement::Evolvement;
use eid_traits::types::Member;

#[derive(Default, Clone)]
pub struct EidDummyEvolvement {
    pub(crate) members: Vec<Member>,
}

impl Evolvement for EidDummyEvolvement {
    fn is_valid_successor(&self, _previous: &Self) -> bool {
        true
    }
}
