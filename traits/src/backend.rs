use crate::client::EidClient;

pub trait EidBackend: Default {
    #[cfg(feature = "test")]
    type ClientProvider: EidClient<BackendProvider = Self>;
}
