use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
struct EidDummyClient {

}

impl EidClient for EidDummyClient {
    type StateProvider = EidDummyState;
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidDummyEvolvement;

    fn default() -> Self { todo!() }
    fn state(&self) -> &<Self as EidClient>::StateProvider { todo!() }

    fn key_store(&self) -> &<Self as EidClient>::KeyStoreProvider { todo!() }
    fn create(&self) -> Result<(), EidError> { todo!() }
    fn add(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
    fn remove(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
    fn update(&self) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
}