pub mod state;
pub mod key_store;
pub mod types;
pub mod evolvement;
mod transcript;

pub trait EidProvider {
    type StateProvider: state::EidState;
    type KeyStoreProvider: key_store::EidKeyStore;

    fn state(&self) -> &Self::StateProvider;

    fn key_store(&self) -> &Self::KeytoreProvider;
}
