use openmls::prelude::MlsMessageOut;

use eid_traits::evolvement::Evolvement;

#[derive(Debug)]
pub struct EidMlsEvolvement {
    pub(crate) message: MlsMessageOut,
    pub(crate) welcome: Option<MlsMessageOut>,
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
