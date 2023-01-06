use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::backend::EidBackend;

pub struct EidMlsBackend {
    pub(crate) mls_backend: &'static OpenMlsRustCrypto,
}

impl EidBackend for EidMlsBackend {}
