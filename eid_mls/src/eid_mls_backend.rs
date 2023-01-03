use eid_traits::backend::EidBackend;
use openmls_rust_crypto::OpenMlsRustCrypto;

pub struct EidMlsBackend {
    pub(crate) mls_backend: OpenMlsRustCrypto,
}

impl EidBackend for EidMlsBackend {}
