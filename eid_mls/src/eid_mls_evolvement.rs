use openmls::framing::MlsMessageIn;
use openmls::prelude::MlsMessageOut;

use eid_traits::evolvement::Evolvement;

#[derive(Debug, Clone)]
pub enum EidMlsEvolvement {
    OUT {
        message: MlsMessageOut,
        welcome: Option<MlsMessageOut>,
    },
    IN {
        message: MlsMessageIn,
    },
}

impl Evolvement for EidMlsEvolvement {
    fn is_valid_successor(&self, _previous: &Self) -> bool {
        todo!()
    }
}
