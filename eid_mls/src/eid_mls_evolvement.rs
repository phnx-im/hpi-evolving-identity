use eid_traits::evolvement::Evolvement;
use openmls::framing::MlsMessageIn;
use openmls::framing::MlsMessageOut;
use openmls::group::StagedCommit;
use openmls::prelude::Welcome;

pub struct EidMlsEvolvement {
    pub(crate) message: MlsMessageOut,
    pub(crate) welcome: Option<Welcome>,
}

impl Clone for EidMlsEvolvement {
    fn clone(&self) -> Self {
        EidMlsEvolvement {
            message: self.message.clone(),
            welcome: self.welcome.clone(),
        }
    }
}

impl Evolvement for EidMlsEvolvement {
    fn is_valid_successor(&self, previous: &Self) -> bool {
        todo!()
    }
}
