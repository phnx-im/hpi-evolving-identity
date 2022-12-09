pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_keystore::EidDummyKeystore;
use eid_dummy::eid_dummy_transcript::EidDummyTranscript;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::state::EidState;
use eid_traits::transcript::Transcript;
use eid_traits::types::EidError;
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
    <C as EidClient>::EvolvementProvider: Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone(), vec![]);

    // Create Alice as a member with a random pk
    let pk_alice = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk_alice);
    let add_alice_evolvement = client.add(alice.clone()).expect("failed to add member");
    client
        .evolve(add_alice_evolvement.clone())
        .expect("Failed to apply state");

    assert!(client.state().verify().unwrap());
    let members = client.state().get_members().expect("failed to get members");
    assert!(members.contains(&alice));
    assert_eq!(2, members.len());

    transcript.add_evolvement(add_alice_evolvement.clone());

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(alice.clone())
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let pk_bob = (0..256).map(|_| rand::random::<u8>()).collect();
    let bob = Member::new(pk_bob);
    let add_bob_evolvement = client.add(bob.clone()).expect("failed to add member");
    client
        .evolve(add_bob_evolvement.clone())
        .expect("Failed to apply state");

    assert!(add_alice_evolvement.is_valid_successor(&add_bob_evolvement));
    transcript.add_evolvement(add_bob_evolvement);

    let members = client.state().get_members().expect("failed to get members");
    assert!(client.state().verify().unwrap());
    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
    <C as EidClient>::EvolvementProvider: Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone(), vec![]);

    let pk = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk);
    let evolvement_add = client.add(alice.clone()).expect("failed to add member");
    client
        .evolve(evolvement_add.clone())
        .expect("Failed to apply state");
    assert!(client.state().verify().unwrap());

    transcript.add_evolvement(evolvement_add.clone());

    let evolvement_remove = client
        .remove(alice.clone())
        .expect("failed to remove member");
    client
        .evolve(evolvement_remove.clone())
        .expect("Failed to apply remove on client state");

    assert!(evolvement_add.is_valid_successor(&evolvement_remove));
    transcript.add_evolvement(evolvement_remove);

    // Try to remove Alice a second time
    let member_not_in_eid_error = client
        .remove(alice.clone())
        .expect_err("Removing non-existent member");
    assert!(matches!(
        member_not_in_eid_error,
        EidError::InvalidMemberError(..)
    ));

    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(state.verify().unwrap());
    assert!(!members.contains(&alice));
    assert_eq!(1, members.len());
}

#[apply(eid_clients)]
fn update<C, T>(client: &mut C, transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider, C::StateProvider>,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone(), vec![]);
    let alice_pk_before_update_1 =
        client.state().get_members().expect("failed to get members")[0].pk();

    let update_evolvement_1 = client.update().expect("Updating client keys failed");
    client
        .evolve(update_evolvement_1.clone())
        .expect("Failed to apply update on client state");
    transcript.add_evolvement(update_evolvement_1.clone());

    let pks_after_update_1 = client
        .state()
        .get_members()
        .expect("failed to get members")
        .iter()
        .map(|m| m.pk())
        .collect::<Vec<_>>();

    assert!(client.state().verify().unwrap());
    assert!(!pks_after_update_1.contains(&alice_pk_before_update_1));
    assert_eq!(1, pks_after_update_1.len());

    // Update Alice a second time
    let alice_pk_before_update_2 = pks_after_update_1[0].clone();
    let update_evolvement_2 = client.update().expect("Updating client keys failed");
    client
        .evolve(update_evolvement_2.clone())
        .expect("Failed to apply update on client state");
    assert!(update_evolvement_1.is_valid_successor(&update_evolvement_2));
    transcript.add_evolvement(update_evolvement_2);

    let pks_after_update_2 = client
        .state()
        .get_members()
        .expect("failed to get members")
        .iter()
        .map(|m| m.pk())
        .collect::<Vec<_>>();

    assert!(client.state().verify().unwrap());
    assert!(!pks_after_update_2.contains(&alice_pk_before_update_2));
    assert_eq!(1, pks_after_update_2.len());
}
