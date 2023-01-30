use tls_codec::{Deserialize, Serialize};

pub trait Evolvement: Clone + Serialize + Deserialize {}
