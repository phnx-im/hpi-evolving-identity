use eid::test_utils::apply;
use eid_dummy::eid_dummy_keystore::EidDummyKeystore;
use eid_traits::client::EidClient;
use eid_traits::state::EidState;


#[apply(eid_clients)]
fn create<V: EidClient>(client: V) {
    let keystore = EidDummyKeystore::default();
    V::create_eid(keystore).expect("creation failed");
    let client_vector = client.state().get_clients().expect("failed to get clients");
    assert_eq!(client_vector.len(), 1)
}

#[apply(eid_clients)]
fn add(client_provider: &impl EidClient) {}
