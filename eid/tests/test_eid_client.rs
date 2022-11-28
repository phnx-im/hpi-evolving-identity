pub use eid_dummy::eid_dummy_client::EidDummyClient;
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

#[apply(eid_clients)]
fn add<T: EidClient>(client: &mut T) {
    let member = Member::default();
    let member_clone = member.clone();
    let evolvement = client.add(member).expect("failed to add member");
    client.state().apply(evolvement).expect("Failed to apply state");

    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(members.contains(&member_clone));
    assert_eq!(1, members.len())
}

#[apply(eid_clients)]
fn remove<T: EidClient>(client: &mut T) {
    let member = Member::default();
    let member_to_remove = member.clone();
    let member_clone = member.clone();
    let evolvement_add = client.add(member).expect("failed to add member");
    client.state().apply(evolvement_add).expect("Failed to apply state");
    assert!(client.state().verify().unwrap());

    let evolvement_remove = client.remove(member_to_remove).expect("failed to remove member");
    client.state().apply(evolvement_remove).expect("Failed to apply state");
    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(!members.contains(&member_clone));
    assert_eq!(0, members.len());
}
