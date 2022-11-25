use crate::evolvement::Evolvement;
use crate::key_store::EidKeyStore;
use crate::state::EidState;
use crate::types::{Member, EidError};

pub trait EidClient: Sized {
    type StateProvider: EidState;
    type KeyStoreProvider: EidKeyStore;
    type EvolvementProvider: Evolvement;

    /// Derive [Default].
    fn default() -> Self;

    fn state(&self) -> &Self::StateProvider;

    fn key_store(&self) -> &Self::KeyStoreProvider;

    /// Create the first [EidState] of an EID by interacting with a PKI. We assume trust on first use on the resulting [EidState].
    fn create(&self) -> Result<(), EidError>;

    /// Create an [Evolvement] to add a member to the EID.
    fn add(&self, member: Member) -> Result<Self::EvolvementProvider, EidError>;

    /// Create an [Evolvement] to remove a member from the EID.
    fn remove(&self, member: Member) -> Result<Self::EvolvementProvider, EidError>;

    /// Create an [Evolvement] to update your own key material.
    fn update(&self) -> Result<Self::EvolvementProvider, EidError>;
}