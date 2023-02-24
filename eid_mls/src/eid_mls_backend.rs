use openmls::prelude::Ciphersuite;
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::backend::EidBackend;

use crate::eid_mls_client::EidMlsClient;

/// # EID MLS Backend
/// Implements [EidBackend] using [openmls]
pub struct EidMlsBackend {
    pub(crate) mls_backend: OpenMlsRustCrypto,
    pub(crate) ciphersuite: Ciphersuite,
}

impl Default for EidMlsBackend {
    fn default() -> Self {
        Self {
            mls_backend: OpenMlsRustCrypto::default(),
            ciphersuite: Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519,
        }
    }
}

impl EidBackend for EidMlsBackend {
    #[cfg(feature = "test")]
    type ClientProvider = EidMlsClient;
}
