use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};

#[derive(Default)]
struct EidDummyClient {

}

impl EidClient for EidDummyClient {
    type StateProvider = Type;
    type KeyStoreProvider = Type;
    type EvolvementProvider = Type;

    fn default() -> Self { todo!() }
    fn state(&self) -> &<Self as EidClient>::StateProvider { todo!() }

    fn key_store(&self) -> &<Self as EidClient>::KeyStoreProvider { todo!() }
    fn create(&self) -> Result<(), EidError> { todo!() }
    fn add(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
    fn remove(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
    fn update(&self) -> Result<<Self as EidClient>::EvolvementProvider, EidError> { todo!() }
}