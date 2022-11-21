use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid::test_utils::apply;


#[apply(eid_clients)]
fn create(client: &impl EidClient) {
    client_provider.create().expect("creation failed");
    let client_vector = client_provider.state().get_clients().expect("failed to get clients");
    assert_eq!(client_vector.len(), 1)
}

#[apply(eid_clients)]
fn add(client_provider: &impl EidClient) {

}