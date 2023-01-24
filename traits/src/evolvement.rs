use tls_codec::{Deserialize, Serialize};

pub trait Evolvement: Clone + Serialize + Deserialize {
    fn is_valid_successor(&self, previous: &Self) -> bool;
}
