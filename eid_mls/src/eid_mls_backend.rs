use openmls::prelude::Ciphersuite;
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::backend::EidBackend;

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

impl EidBackend for EidMlsBackend {}
