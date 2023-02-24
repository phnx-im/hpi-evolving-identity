#[cfg(feature = "test")]
use crate::client::EidClient;

/// # EidBackend
/// Represents the interface to a provider of cryptographic functions.
pub trait EidBackend: Default {
    #[cfg(feature = "test")]
    type ClientProvider: EidClient<BackendProvider = Self>;
}
