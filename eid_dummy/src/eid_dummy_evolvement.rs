use eid_traits::evolvement::Evolvement;
use eid_traits::types::Member;

#[derive(Debug, Clone)]
pub enum EidDummyEvolvement {
    Add { members: Vec<Member> },
    Update { members: Vec<Member> },
    Remove { members: Vec<Member> },
}
impl Default for EidDummyEvolvement {
    fn default() -> Self {
        Self::Add { members: vec![] }
    }
}

impl Evolvement for EidDummyEvolvement {
    fn is_valid_successor(&self, _previous: &Self) -> bool {
        true
    }
}
