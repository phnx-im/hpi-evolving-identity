use openmls_rust_crypto::OpenMlsRustCrypto;
#[cfg(feature = "test")]
use openmls_traits::types::Ciphersuite;

use eid_traits::backend::EidBackend;

#[cfg(feature = "test")]
use crate::eid_mls_client::EidMlsClient;

/// # EID MLS Backend
/// Implements [EidBackend] using [openmls]
#[cfg_attr(not(feature = "test"), derive(Default))]
pub struct EidMlsBackend {
    pub(crate) mls_backend: OpenMlsRustCrypto,
    #[cfg(feature = "test")]
    pub ciphersuite: Ciphersuite,
}

#[cfg(feature = "test")]
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
