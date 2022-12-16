use crate::evolvement::Evolvement;
use crate::key_store::EidKeyStore;
use crate::state::EidState;
use crate::types::{EidError, Member};
use std::fmt::Debug;

pub trait EidClient<'a> {
    type KeyStoreProvider: EidKeyStore + Default;
    type EvolvementProvider: Evolvement + Debug;
    type StateProvider: EidState<Self::EvolvementProvider>;

    fn state(&mut self) -> &mut Self::StateProvider;

    fn key_store(&self) -> &Self::KeyStoreProvider;

    fn pk(&self) -> &[u8];

    /// Create the first [EidState] of an EID by interacting with a PKI. We assume trust on first use on the resulting [EidState].
    fn create_eid(keystore: &'a Self::KeyStoreProvider) -> Result<Self, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to add a member to the EID.
    fn add(&self, member: &Member) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to remove a member from the EID.
    fn remove(&self, member: &Member) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized;

    /// Create an [Evolvement] to update your own key material.
    fn update(&mut self) -> Result<Self::EvolvementProvider, EidError>;

    /// Apply an [evolvement::Evolvement], changing the client's state. If the [evolvement::Evolvement]
    /// is invalid, return an [EidError].
    fn evolve(&mut self, evolvement: &Self::EvolvementProvider) -> Result<(), EidError>;
}
