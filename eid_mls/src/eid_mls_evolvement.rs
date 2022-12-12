use eid_traits::evolvement::Evolvement;
use eid_traits::state::EidState;
use openmls::framing::MlsMessageIn;
use openmls::group::StagedCommit;

pub struct EidMlsEvolvement {
    pub(crate) commit: StagedCommit,
    pub(crate) message: MlsMessageIn,
}

impl Clone for EidMlsEvolvement {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Evolvement for EidMlsEvolvement {
    fn is_valid_successor(&self, previous: &Self) {}
}
