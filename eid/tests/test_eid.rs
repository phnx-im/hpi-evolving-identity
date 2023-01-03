#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;

pub use rstest::*;
pub use rstest_reuse::{self, *};

use eid_dummy::eid_dummy_backend::EidDummyBackend;
pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_transcript::EidDummyTranscript;
// use eid_mls::eid_mls_client::EidMlsClient;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::state::EidState;
use eid_traits::transcript::Transcript;
use eid_traits::types::EidError;
use eid_traits::types::Member;

lazy_static! {
    static ref DUMMY_BACKEND: EidDummyBackend = EidDummyBackend::default();
    // static ref MLS_BACKEND: = EidMlsBackend = EidMlsBackend::default();
}

// case::EIDMls(& mut EidMlsClient::create_eid(&MLS_BACKEND).expect("creation failed"), EidDummyTranscript::default(), &MLS_BACKEND),
#[template]
#[rstest(client, _transcript, backend,
case::EIDDummy(& mut EidDummyClient::create_eid(& DUMMY_BACKEND).expect("creation failed"), EidDummyTranscript::default(), & DUMMY_BACKEND),
)]
#[allow(non_snake_case)]
pub fn eid_clients<C, T, B>(client: &mut C, _transcript: T, backend: &B)
where
    C: EidClient + EidClient<BackendProvider = B>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient>::KeyStoreProvider: Default,
    B: EidBackend,
{
}

#[apply(eid_clients)]
fn create<'a, C, T, B>(client: &mut C, _transcript: T, backend: &B)
where
    C: EidClient,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient>::StateProvider: Debug,
    <C as EidClient>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<<C as EidClient>::EvolvementProvider>>::StateProvider:
        From<<C as EidClient>::StateProvider> + Debug,
    <C as EidClient>::StateProvider: 'a,
{
    let members = client.state().get_members().expect("failed to get members");
    // create transcript, trusting the client's state
    let transcript = T::new(client.state().clone().into(), vec![]); //  T::new(T::StateProvider::from(client.state().clone()), vec![]);
    assert_eq!(
        transcript.trusted_state(),
        client.state().clone().into(),
        "initial states of transcript and client do not match"
    );
    assert_eq!(members.len(), 1);
}

#[apply(eid_clients)]
fn add<'a, C, T, B>(client: &'a mut C, _transcript: T, backend: &B)
where
    C: EidClient + EidClient<BackendProvider = B>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient>::StateProvider: Debug,
    <C as EidClient>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<<C as EidClient>::EvolvementProvider>>::StateProvider:
        From<<C as EidClient>::StateProvider> + Debug,
    <C as EidClient>::StateProvider: 'a,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);

    // Create Alice as a member with a random pk
    let pk_alice = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk_alice);
    let add_alice_evolvement = client.add(&alice, backend).expect("failed to add member");
    client
        .evolve(&add_alice_evolvement, backend)
        .expect("Failed to apply state");

    let members = client.state().get_members().expect("failed to get members");
    assert!(members.contains(&alice));
    assert_eq!(2, members.len());

    transcript.add_evolvement(add_alice_evolvement.clone());

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(&alice, backend)
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let pk_bob = (0..256).map(|_| rand::random::<u8>()).collect();
    let bob = Member::new(pk_bob);
    let add_bob_evolvement = client.add(&bob, backend).expect("failed to add member");
    client
        .evolve(&add_bob_evolvement, backend)
        .expect("Failed to apply state");

    assert!(add_alice_evolvement.is_valid_successor(&add_bob_evolvement));
    transcript.add_evolvement(add_bob_evolvement.clone());

    let members = client.state().get_members().expect("failed to get members");
    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<'a, C, T, B>(client: &mut C, _transcript: T, backend: &B)
where
    C: EidClient + EidClient<BackendProvider = B>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient>::StateProvider: Debug,
    <C as EidClient>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<<C as EidClient>::EvolvementProvider>>::StateProvider:
        From<<C as EidClient>::StateProvider> + Debug,
    <C as EidClient>::StateProvider: 'a,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);

    let pk = (0..256).map(|_| rand::random::<u8>()).collect();
    let alice = Member::new(pk);
    let evolvement_add = client.add(&alice, backend).expect("failed to add member");
    client
        .evolve(&evolvement_add, backend)
        .expect("Failed to apply state");

    transcript.add_evolvement(evolvement_add.clone());

    let evolvement_remove = client
        .remove(&alice, backend)
        .expect("failed to remove member");
    client
        .evolve(&evolvement_remove, backend)
        .expect("Failed to apply remove on client state");

    assert!(evolvement_add.is_valid_successor(&evolvement_remove));
    transcript.add_evolvement(evolvement_remove.clone());

    // Try to remove Alice a second time
    let member_not_in_eid_error = client
        .remove(&alice, backend)
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
fn update<'a, C, T, B>(client: &mut C, _transcript: T, backend: &B)
where
    C: EidClient + EidClient<BackendProvider = B>,
    T: Transcript<C::EvolvementProvider>,
    <C as EidClient>::StateProvider: Debug,
    <C as EidClient>::EvolvementProvider: Debug,
    // require that a transcript state can be created from a client state
    <T as Transcript<<C as EidClient>::EvolvementProvider>>::StateProvider:
        From<<C as EidClient>::StateProvider> + Debug,
    <C as EidClient>::StateProvider: 'a,
{
    // Create transcript, trusting the client's state
    let mut transcript = T::new(client.state().clone().into(), vec![]);
    let alice_pk_before_update_1 =
        client.state().get_members().expect("failed to get members")[0].pk();

    let update_evolvement_1 = client.update(backend).expect("Updating client keys failed");
    client
        .evolve(&update_evolvement_1, backend)
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
    let update_evolvement_2 = client.update(backend).expect("Updating client keys failed");
    client
        .evolve(&update_evolvement_2, backend)
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
