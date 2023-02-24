use tls_codec::{Deserialize, Serialize};

/// # Evolvement
/// Represents one change in an EID.
/// The History of an EID is the log of all evolvements.
pub trait Evolvement: Clone + Serialize + Deserialize {}
