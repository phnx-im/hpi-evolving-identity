use std::collections::HashMap;
use std::sync::RwLock;

use eid_traits::key_store::{EidKeyStore, FromKeyStoreValue, ToKeyStoreValue};

/// Copied from OpenMlsKeyStore: https://github.com/openmls/openmls/tree/3248d1e2a91a79c6e7140487cb85e1bb379de274/memory_keystore.
#[derive(Default)]
pub struct EidDummyKeystore {
    values: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
}

impl EidKeyStore for EidDummyKeystore {
    type Error = Error;

    fn store<V: ToKeyStoreValue>(&self, k: &[u8], v: &V) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        let value = v
            .to_key_store_value()
            .map_err(|_| Error::SerializationError)?;
        // We unwrap here, because this is the only function claiming a write
        // lock on `credential_bundles`. It only holds the lock very briefly and
        // should not panic during that period.
        let mut values = self.values.write().unwrap();
        values.insert(k.to_vec(), value);
        Ok(())
    }

    fn read<V: FromKeyStoreValue>(&self, k: &[u8]) -> Option<V>
    where
        Self: Sized,
    {
        // We unwrap here, because the two functions claiming a write lock on
        // `init_key_package_bundles` (this one and `generate_key_package_bundle`) only
        // hold the lock very briefly and should not panic during that period.
        let values = self.values.read().unwrap();
        if let Some(value) = values.get(k) {
            V::from_key_store_value(value).ok()
        } else {
            None
        }
    }

    fn delete(&self, k: &[u8]) -> Result<(), Self::Error> {
        // We just delete both ...
        let mut values = self.values.write().unwrap();
        values.remove(k);
        Ok(())
    }
}

/// Errors thrown by the key store.
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("The key store does not allow storing serialized values.")]
    UnsupportedValueTypeBytes,
    #[error("Updating is not supported by this key store.")]
    UnsupportedMethod,
    #[error("Error serializing value.")]
    SerializationError,
    #[error("Error deserializing value.")]
    DeserializationError,
}
