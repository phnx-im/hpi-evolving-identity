pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_keystore::EidDummyKeystore;
use eid_dummy::eid_dummy_transcript::EidDummyTranscript;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::state::EidState;
use eid_traits::transcript::Transcript;
use eid_traits::types::Member;
pub use rstest::*;
pub use rstest_reuse::{self, *};
use std::fmt::Debug;

#[template]
#[rstest(client, transcript,
    case::EIDDummy(&mut EidDummyClient::create_eid(EidDummyKeystore::default()).expect("creation failed"), EidDummyTranscript::default()),
)]
#[allow(non_snake_case)]
pub fn eid_clients<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
{
}

#[apply(eid_clients)]
fn create<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
    <C as EidClient>::StateProvider: Debug,
{
    let members = client.state().get_members().expect("failed to get members");
    // create transcript, trusting the client's state
    let transcript = T::new(client.state().clone(), vec![]);
    assert_eq!(
        &mut transcript.trusted_state(),
        client.state(),
        "initial states of transcript and client do not match"
    );
    assert_eq!(members.len(), 1);
    assert!(client.state().verify().unwrap());
}

#[apply(eid_clients)]
fn add<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
{
    // create transcript, trusting the client's state
    T::new(client.state().clone(), vec![]);
    let pk = (0..256).map(|_| rand::random::<u8>()).collect();
    let member = Member::new(pk);
    let member_clone = member.clone();
    let evolvement = client.add(member).expect("failed to add member");
    client
        .state()
        .apply(evolvement)
        .expect("Failed to apply state");

    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(members.contains(&member_clone));
    assert_eq!(2, members.len())
}

#[apply(eid_clients)]
fn remove<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
{
    let pk = (0..256).map(|_| rand::random::<u8>()).collect();
    let member = Member::new(pk);
    let member_to_remove = member.clone();
    let member_clone = member.clone();
    let evolvement_add = client.add(member).expect("failed to add member");
    client
        .state()
        .apply(evolvement_add)
        .expect("Failed to apply state");
    assert!(client.state().verify().unwrap());

    let evolvement_remove = client
        .remove(member_to_remove)
        .expect("failed to remove member");
    client
        .state()
        .apply(evolvement_remove)
        .expect("Failed to apply remove on client state");
    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(!members.contains(&member_clone));
    assert_eq!(1, members.len());
}

#[apply(eid_clients)]
fn update<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
{
    let member = client.state().get_members().expect("failed to get members")[0].clone();

    let update_evolvement = client.update().expect("Updating client keys failed");
    client
        .state()
        .apply(update_evolvement)
        .expect("Failed to apply update on client state");
    let new_members = client.state().get_members().expect("failed to get members");

    assert!(client.state().verify().unwrap());
    assert!(!new_members.contains(&member));
    assert_eq!(1, new_members.len());
}
