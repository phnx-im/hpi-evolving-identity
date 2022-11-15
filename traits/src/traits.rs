use crate::types::EidError;

pub mod state;
pub mod key_store;
pub mod types;
pub mod evolvement;
pub mod transcript;

pub trait EidClient {
    type StateProvider: state::EidState;
    type KeyStoreProvider: key_store::EidKeyStore;

    fn state(&self) -> &Self::StateProvider;

    fn key_store(&self) -> &Self::KeyStoreProvider;

    /// Create the first [EidState] of an EID by interacting with a PKI. We assume trust on first use on the resulting [EidState].
    fn create(&self) -> Result<(), EidError>;

    /// Create an [Evolvement] to add a client to the EID.
    fn add(&self) -> Result<Self::Evolvement, EidError>;

    /// Create an [Evolvement] to remove a client from the EID.
    fn remove(&self) -> Result<Self::Evolvement, EidError>;

    /// Create an [Evolvement] to update your own key material.
    fn update(&self) -> Result<Self::Evolvement, EidError>;
}
