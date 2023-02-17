use crate::client::EidClient;

/// Represents the interface to a provider of cryptographic functions.
pub trait EidBackend: Default {
    #[cfg(feature = "test")]
    type ClientProvider: EidClient<BackendProvider = Self>;
}
