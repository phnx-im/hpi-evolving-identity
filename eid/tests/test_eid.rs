#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;

pub use rstest::*;
pub use rstest_reuse::{self, *};

pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_keystore::EidDummyKeystore;
use eid_dummy::eid_dummy_transcript::EidDummyTranscript;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::state::EidState;
use eid_traits::transcript::Transcript;
use eid_traits::types::EidError;
use eid_traits::types::Member;

lazy_static! {
    static ref DUMMY_KEYSTORE: EidDummyKeystore = EidDummyKeystore::default();
}

#[template]
#[rstest(client, _transcript,
case::EIDDummy(& mut EidDummyClient::create_eid(& DUMMY_KEYSTORE).expect("creation failed"), EidDummyTranscript::default()),
case::EIDMls(& mut EidMlsClient::create_eid(& DUMMY_KEYSTORE).expect("creation failed"), EidDummyTranscript::default()),
)]
#[allow(non_snake_case)]
pub fn eid_clients<'a, C, T>(client: &mut C, _transcript: T)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient<'a>>::KeyStoreProvider: Default,
{
}

#[apply(eid_clients)]
fn create<'a, C, T>(client: &mut C, _transcript: T)
where
    C: EidClient<'a>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient<'a>>::StateProvider: Debug,
    <C as EidClient<'a>>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<C::EvolvementProvider>>::StateProvider: From<C::StateProvider> + Debug,
{
    let members = client.state().get_members().expect("failed to get members");
    // create transcript, trusting the client's state
    let transcript = T::new(client.state().clone().into(), vec![]); //  T::new(T::StateProvider::from(client.state().clone()), vec![]);
    assert_eq!(
        transcript.trusted_state().clone(),
        client.state().clone().into(),
        "initial states of transcript and client do not match"
    );
    assert_eq!(members.len(), 1);
}

#[apply(eid_clients)]
fn add<'a, C, T>(client: &mut C, _transcript: T)
where
    C: EidClient<'a>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient<'a>>::StateProvider: Debug,
    <C as EidClient<'a>>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<C::EvolvementProvider>>::StateProvider: From<C::StateProvider> + Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);

    // Create Alice as a member with a random pk
    let pk_alice = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk_alice);
    let add_alice_evolvement = client.add(&alice).expect("failed to add member");
    client
        .evolve(&add_alice_evolvement)
        .expect("Failed to apply state");

    let members = client.state().get_members().expect("failed to get members");
    assert!(members.contains(&alice));
    assert_eq!(2, members.len());

    transcript.add_evolvement(add_alice_evolvement.clone());

    // Try to add Alice a second time
    let member_in_eid_error = client.add(&alice).expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let pk_bob = (0..256).map(|_| rand::random::<u8>()).collect();
    let bob = Member::new(pk_bob);
    let add_bob_evolvement = client.add(&bob).expect("failed to add member");
    client
        .evolve(&add_bob_evolvement)
        .expect("Failed to apply state");

    assert!(add_alice_evolvement.is_valid_successor(&add_bob_evolvement));
    transcript.add_evolvement(add_bob_evolvement.clone());

    let members = client.state().get_members().expect("failed to get members");
    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<'a, C, T>(client: &mut C, _transcript: T)
where
    C: EidClient<'a>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient<'a>>::StateProvider: Debug,
    <C as EidClient<'a>>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<C::EvolvementProvider>>::StateProvider: From<C::StateProvider> + Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);

    let pk = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk);
    let evolvement_add = client.add(&alice).expect("failed to add member");
    client
        .evolve(&evolvement_add)
        .expect("Failed to apply state");

    transcript.add_evolvement(evolvement_add.clone());

    let evolvement_remove = client.remove(&alice).expect("failed to remove member");
    client
        .evolve(&evolvement_remove)
        .expect("Failed to apply remove on client state");

    assert!(evolvement_add.is_valid_successor(&evolvement_remove));
    transcript.add_evolvement(evolvement_remove.clone());

    // Try to remove Alice a second time
    let member_not_in_eid_error = client
        .remove(&alice)
        .expect_err("Removing non-existent member");
    assert!(matches!(
        member_not_in_eid_error,
        EidError::InvalidMemberError(..)
    ));

    let state = client.state();
    let members = state.get_members().expect("failed to get members");
    assert!(!members.contains(&alice));
    assert_eq!(1, members.len());
}

#[apply(eid_clients)]
fn update<'a, C, T>(client: &mut C, _transcript: T)
where
    C: EidClient<'a>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient<'a>>::StateProvider: Debug,
    <C as EidClient<'a>>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<C::EvolvementProvider>>::StateProvider: From<C::StateProvider> + Debug,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);
    let alice_pk_before_update_1 =
        client.state().get_members().expect("failed to get members")[0].pk();

    let update_evolvement_1 = client.update().expect("Updating client keys failed");
    client
        .evolve(&update_evolvement_1)
        .expect("Failed to apply update on client state");
    transcript.add_evolvement(update_evolvement_1.clone());

    let pks_after_update_1 = client
        .state()
        .get_members()
        .expect("failed to get members")
        .iter()
        .map(|m| m.pk())
        .collect::<Vec<_>>();

    assert!(!pks_after_update_1.contains(&alice_pk_before_update_1));
    assert_eq!(1, pks_after_update_1.len());

    // Update Alice a second time
    let alice_pk_before_update_2 = pks_after_update_1[0].clone();
    let update_evolvement_2 = client.update().expect("Updating client keys failed");
    client
        .evolve(&update_evolvement_2)
        .expect("Failed to apply update on client state");
    assert!(update_evolvement_1.is_valid_successor(&update_evolvement_2));
    transcript.add_evolvement(update_evolvement_2.clone());

    let pks_after_update_2 = client
        .state()
        .get_members()
        .expect("failed to get members")
        .iter()
        .map(|m| m.pk())
        .collect::<Vec<_>>();

    assert!(!pks_after_update_2.contains(&alice_pk_before_update_2));
    assert_eq!(1, pks_after_update_2.len());
}
