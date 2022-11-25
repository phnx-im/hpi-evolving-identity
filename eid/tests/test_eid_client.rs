pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_keystore::EidDummyKeystore;
use eid_traits::client::EidClient;
use eid_traits::state::EidState;
pub use rstest::*;
pub use rstest_reuse::{self, *};

#[template]
#[rstest(client, case::EIDDummy(&EidDummyClient::default()),)]
pub fn eid_clients(client: &impl EidClient) {}

#[apply(eid_clients)]
fn create<T: EidClient>(client: &T) {
    let keystore = T::KeyStoreProvider::default();
    let client = T::create_eid(keystore).expect("creation failed");
    let client_vector = client.state().get_clients().expect("failed to get clients");
    assert_eq!(client_vector.len(), 1)
}
