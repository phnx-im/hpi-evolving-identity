use openmls_rust_crypto::OpenMlsRustCrypto;
#[cfg(feature = "test")]
use openmls_traits::types::Ciphersuite;

use eid_traits::backend::EidBackend;

#[cfg(feature = "test")]
use crate::eid_mls_client::EidMlsClient;

/// # EID MLS Backend
/// Implements [EidBackend] using [openmls]
pub struct EidMlsBackend {
    pub(crate) mls_backend: OpenMlsRustCrypto,
    #[cfg(feature = "test")]
    pub(crate) ciphersuite: Ciphersuite,
}

impl Default for EidMlsBackend {
    #[cfg(feature = "test")]
    fn default() -> Self {
        Self {
            mls_backend: OpenMlsRustCrypto::default(),
            ciphersuite: Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519,
        }
    }

    #[cfg(not(feature = "test"))]
    fn default() -> Self {
        Self {
            mls_backend: OpenMlsRustCrypto::default(),
        }
    }
}

impl EidBackend for EidMlsBackend {
    #[cfg(feature = "test")]
    type ClientProvider = EidMlsClient;
}
