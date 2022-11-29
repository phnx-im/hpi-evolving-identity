use crate::eid_dummy_keystore::Error;
use eid_traits::key_store::{FromKeyStoreValue, ToKeyStoreValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KeyPair {
    pub(crate) sk: Vec<u8>,
    pub(crate) pk: Vec<u8>,
}

impl FromKeyStoreValue for KeyPair {
    type Error = Error;

    fn from_key_store_value(ksv: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(ksv).map_err(|_| Error::DeserializationError)
    }
}

impl ToKeyStoreValue for KeyPair {
    type Error = Error;

    fn to_key_store_value(&self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(self).map_err(|_| Error::SerializationError)
    }
}
