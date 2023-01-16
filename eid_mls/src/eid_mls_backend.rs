use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::backend::EidBackend;

pub struct EidMlsBackend {
    pub(crate) mls_backend: OpenMlsRustCrypto,
}

impl Default for EidMlsBackend {
    fn default() -> Self {
        Self {
            mls_backend: OpenMlsRustCrypto::default(),
        }
    }
}

impl EidBackend for EidMlsBackend {}
