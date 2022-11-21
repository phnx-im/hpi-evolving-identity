use eid_traits::client::EidClient;

#[derive(Default)]
struct EidMlsClient {

}

impl EidClient for EidMlsClient {
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