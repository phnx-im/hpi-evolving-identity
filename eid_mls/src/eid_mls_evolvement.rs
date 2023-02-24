use std::io::{Read, Write};

use openmls::prelude::{MlsMessageIn, MlsMessageOut};
use tls_codec::{Deserialize, Error as TlsError, Serialize, Size};

use eid_traits::evolvement::Evolvement;

/// # EidMlsEvolvement
/// Implementation of [Evolvement] using [openmls].
/// To keep the API as simple as possible, [EidMlsEvolvement] doesn't introduce extra types for in- and outbound [Evolvement]s.
#[derive(Debug, Clone)]
pub enum EidMlsEvolvement {
    OUT {
        message: MlsMessageOut,
        welcome: Option<MlsMessageOut>,
    },
    IN {
        message: MlsMessageIn,
        welcome: Option<MlsMessageIn>,
    },
}

impl Serialize for EidMlsEvolvement {
    fn tls_serialize<W: Write>(&self, writer: &mut W) -> Result<usize, TlsError> {
        if let Self::OUT { message, welcome } = self {
            let mut bytes_written = 0;
            let msg_ser = message.tls_serialize_detached()?;
            bytes_written += writer.write(msg_ser.as_slice())?;

            let welcome_ser = welcome.tls_serialize_detached()?;
            bytes_written += writer.write(welcome_ser.as_slice())?;

            Ok(bytes_written)
        } else {
            Err(TlsError::EncodingError(String::from(
                "Expected EidMlsEvolvement::OUT, got ::IN",
            )))
        }
    }
}

impl Size for EidMlsEvolvement {
    fn tls_serialized_len(&self) -> usize {
        match self {
            Self::OUT { message, welcome } => {
                let len = message.tls_serialized_len();
                let welcome_len = match welcome {
                    None => 0,
                    Some(msg) => msg.tls_serialized_len(),
                };
                len + welcome_len
            }
            Self::IN { message, welcome } => {
                let len = message.tls_serialized_len();
                let welcome_len = match welcome {
                    None => 0,
                    Some(msg) => msg.tls_serialized_len(),
                };
                len + welcome_len
            }
        }
    }
}

impl Deserialize for EidMlsEvolvement {
    fn tls_deserialize<R: Read>(bytes: &mut R) -> Result<Self, TlsError>
    where
        Self: Sized,
    {
        let message = MlsMessageIn::tls_deserialize(bytes)?;
        let welcome = Option::<MlsMessageIn>::tls_deserialize(bytes)?;
        Ok(Self::IN { message, welcome })
    }
}

impl Evolvement for EidMlsEvolvement {}
