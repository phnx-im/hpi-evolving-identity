use openmls::framing::{MlsMessageIn, ProtocolMessage, PublicMessage};
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
    fn is_valid_successor(&self, previous: &Self) -> bool {
        todo!()
    }
}
