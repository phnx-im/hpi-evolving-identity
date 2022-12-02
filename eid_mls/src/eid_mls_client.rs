use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};

#[derive(Default)]
struct EidMlsClient {}

impl EidClient for EidMlsClient {
    type StateProvider = Type; // Todo
    type KeyStoreProvider = Type; // Todo
    type EvolvementProvider = Type; // Todo

    fn state(&self) -> &<Self as EidClient>::StateProvider {
        todo!()
    }

    fn key_store(&self) -> &<Self as EidClient>::KeyStoreProvider {
        todo!()
    }
    fn create_eid(keystore: Self::KeyStoreProvider) -> Result<(), EidError> {
        todo!()
    }
    fn add(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> {
        todo!()
    }
    fn remove(&self, _: Member) -> Result<<Self as EidClient>::EvolvementProvider, EidError> {
        todo!()
    }
    fn update(&self) -> Result<<Self as EidClient>::EvolvementProvider, EidError> {
        todo!()
    }

    fn pk(&self) -> &Vec<u8> {
        todo!()
    }
}
