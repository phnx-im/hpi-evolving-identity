#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;

pub use rstest::*;
pub use rstest_reuse::{self, *};
use tls_codec::{Deserialize, Serialize};

use eid_dummy::eid_dummy_backend::EidDummyBackend;
pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_member::EidDummyMember;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::member::Member;
use eid_traits::transcript::{EidExportedTranscriptState, EidTranscript};
use eid_traits::types::EidError;

lazy_static! {
    static ref DUMMY_BACKEND: EidDummyBackend = EidDummyBackend::default();
    static ref MLS_BACKEND: EidMlsBackend = EidMlsBackend::default();
}

#[template]
#[rstest(client, backend,
case::EidDummy(& mut EidDummyClient::create_eid(&EidDummyMember::new("test_key".as_bytes().to_vec()),& DUMMY_BACKEND).expect("creation failed"), & DUMMY_BACKEND),
case::EidMls(& mut EidMlsClient::create_eid(&EidMlsClient::generate_initial_id("test_id".into(), &MLS_BACKEND), & MLS_BACKEND).expect("creation failed"), & MLS_BACKEND),
)]
#[allow(non_snake_case)]
pub fn eid_clients<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
}

#[apply(eid_clients)]
fn add<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    let mut transcript = build_transcript(client, backend);

    // member list length unchanged before evolving
    let members = client.get_members();

    assert_eq!(0, members.len());

    let cross_sign_evolvement_out = client
        .cross_sign_membership(backend)
        .expect("Cross signing failed");
    let cross_sign_evolvement_in: C::EvolvementProvider =
        simulate_transfer(&cross_sign_evolvement_out);

    client
        .evolve(cross_sign_evolvement_in.clone(), backend)
        .expect("Failed to apply state");
    let members = client.get_members();
    assert_eq!(1, members.len());

    // Create Alice as a member with a random pk
    let alice = C::generate_initial_id("alice".into(), backend);
    let add_alice_evolvement_out = client.add(&alice, backend).expect("failed to add member");

    let add_alice_evolvement_in: C::EvolvementProvider =
        simulate_transfer(&add_alice_evolvement_out);

    client
        .evolve(add_alice_evolvement_in.clone(), backend)
        .expect("Failed to apply state");

    let members = client.get_members();
    assert!(members.contains(&alice));
    assert_eq!(2, members.len());

    transcript
        .add_evolvement(add_alice_evolvement_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), members);

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(&alice, backend)
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let bob = C::generate_initial_id("bob".into(), backend);
    let add_bob_evolvement_out = client.add(&bob, backend).expect("failed to add member");
    let add_bob_evolvement_in: C::EvolvementProvider = simulate_transfer(&add_bob_evolvement_out);
    client
        .evolve(add_bob_evolvement_in.clone(), backend)
        .expect("Failed to apply state");

    assert!(add_alice_evolvement_in.is_valid_successor(&add_bob_evolvement_in));
    transcript
        .add_evolvement(add_bob_evolvement_in.clone(), backend)
        .expect("Failed to add evolvement");

    let members = client.get_members();
    assert_eq!(transcript.get_members(), members);
    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    let mut transcript = build_transcript(client, backend);

    let alice = C::generate_initial_id("alice".into(), backend);
    let evolvement_add_out = client.add(&alice, backend).expect("failed to add member");
    let evolvement_add_in: C::EvolvementProvider = simulate_transfer(&evolvement_add_out);

    client
        .evolve(evolvement_add_in.clone(), backend)
        .expect("Failed to apply state");

    transcript
        .add_evolvement(evolvement_add_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), client.get_members());

    let evolvement_remove_out = client
        .remove(&alice, backend)
        .expect("failed to remove member");
    let evolvement_remove_in: C::EvolvementProvider = simulate_transfer(&evolvement_remove_out);
    client
        .evolve(evolvement_remove_in.clone(), backend)
        .expect("Failed to apply remove on client state");

    assert!(evolvement_add_in.is_valid_successor(&evolvement_remove_in));
    transcript
        .add_evolvement(evolvement_remove_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), client.get_members());

    // Try to remove Alice a second time
    let member_not_in_eid_error = client
        .remove(&alice, backend)
        .expect_err("Removing non-existent member");
    assert!(matches!(
        member_not_in_eid_error,
        EidError::InvalidMemberError(..)
    ));
    let members = client.get_members();
    assert!(!members.contains(&alice));
    assert_eq!(1, members.len());
}

#[apply(eid_clients)]
fn update<C, B>(client: &mut C, backend: &B)
where
    C: EidClient<BackendProvider = B>,
    C::EvolvementProvider: Debug,
{
    let mut transcript = build_transcript(client, backend);

    let alice_before_update_1 = &client.get_members()[0];

    let update_evolvement_1_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_1_in: C::EvolvementProvider = simulate_transfer(&update_evolvement_1_out);
    client
        .evolve(update_evolvement_1_in.clone(), backend)
        .expect("Failed to apply update on client state");
    transcript
        .add_evolvement(update_evolvement_1_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_1 = client.get_members();

    assert!(!members_after_update_1.contains(alice_before_update_1));
    assert_eq!(1, members_after_update_1.len());

    // Update Alice a second time
    let alice_before_update_2 = &members_after_update_1[0];
    let update_evolvement_2_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_2_in: C::EvolvementProvider = simulate_transfer(&update_evolvement_2_out);
    client
        .evolve(update_evolvement_2_in.clone(), backend)
        .expect("Failed to apply update on client state");
    assert!(update_evolvement_1_in.is_valid_successor(&update_evolvement_2_in));
    transcript
        .add_evolvement(update_evolvement_2_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_2 = client.get_members();

    assert!(!members_after_update_2.contains(alice_before_update_2));
    assert_eq!(1, members_after_update_2.len());
}

/// Create transcript, trusting the client's state
#[cfg(feature = "test")]
fn build_transcript<C, B>(client: &C, backend: &B) -> C::TranscriptProvider
where
    C: EidClient<BackendProvider = B>,
{
    let exported_state = client
        .export_transcript_state(backend)
        .expect("failed to export transcript state");

    let imported_state: C::ExportedTranscriptStateProvider = simulate_transfer(&exported_state);

    C::TranscriptProvider::new(
        imported_state
            .into_transcript_state(backend)
            .expect("failed to create transcript state"),
        vec![],
        backend,
    )
    .expect("Failed to create transcript")
}

/// Simulate transfer over the wire by simply serializing and deserializing once.
#[cfg(feature = "test")]
fn simulate_transfer<I: Serialize, O: Deserialize>(input: &I) -> O {
    let serialized = input.tls_serialize_detached().expect("Failed to serialize");
    O::tls_deserialize(&mut serialized.as_slice()).expect("Failed to deserialize")
}

#[test]
fn test_mls_add() {
    let backend = &MLS_BACKEND;
    let client = &mut EidMlsClient::create_eid(
        &EidMlsClient::generate_initial_id(String::from("client01"), &MLS_BACKEND),
        &MLS_BACKEND,
    )
    .expect("creation failed");
    add(client, backend);
}
