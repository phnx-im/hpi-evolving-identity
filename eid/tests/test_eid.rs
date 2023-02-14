use std::fmt::Debug;

use openmls::prelude::SignatureScheme;
use openmls_basic_credential::SignatureKeyPair;
pub use rstest::*;
pub use rstest_reuse::{self, *};
use tls_codec::{Deserialize, Serialize};

use eid_dummy::eid_dummy_backend::EidDummyBackend;
pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_dummy::eid_dummy_member::EidDummyMember;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_mls::eid_mls_member::EidMlsMember;
use eid_traits::backend::EidBackend;
use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
use eid_traits::member::Member;
use eid_traits::transcript::{EidExportedTranscriptState, EidTranscript};
use eid_traits::types::EidError;

#[template]
#[rstest(backend,
case::EidDummy(& EidDummyBackend::default()),
case::EidMls(& EidMlsBackend::default()),
)]
#[allow(non_snake_case)]
pub fn eid_clients<B: EidBackend>(backend: &B) {}

#[apply(eid_clients)]
fn add<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    // member list length unchanged before evolving
    let members_initial = client.get_members();

    assert_eq!(0, members_initial.len());

    let cross_sign_evolvement = cross_sign(client, backend);

    let members_after_cross_sign = client.get_members();
    assert_eq!(1, members_after_cross_sign.len());

    // Create Alice as a member with a random pk
    let (alice, alice_kp) = B::ClientProvider::generate_initial_member("alice".into(), backend);
    let (add_alice_evolvement, cross_sign_alice_evolvement) =
        add_and_cross_sign(client, alice.clone(), alice_kp, backend);

    let members_after_alice_cross_sign = client.get_members();
    assert!(members_after_alice_cross_sign.contains(&alice));
    assert_eq!(2, members_after_alice_cross_sign.len());

    // TODO transcript
    //     .add_evolvement(add_evolvement.clone(), backend)
    //     .expect("Failed to add evolvement");
    // TODO transcript
    //     .add_evolvement(cross_sign_evolvement.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), members_after_alice_cross_sign);

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(&alice, backend)
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let (bob, bob_kp) = B::ClientProvider::generate_initial_member("bob".into(), backend);
    add_and_cross_sign(client, bob.clone(), bob_kp, backend);

    let members = client.get_members();

    // TODO transcript
    //     .add_evolvement(add_bob_evolvement_in.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), members);

    assert!(members.contains(&bob));
    assert_eq!(3, members.len())
}

#[apply(eid_clients)]
fn remove<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    let cross_sign_evolvement = cross_sign(client, backend);

    let (alice, keypair_alice) =
        B::ClientProvider::generate_initial_member("alice".into(), backend);
    let (add_alice_evolvement, cross_sign_alice_evolvement) =
        add_and_cross_sign(client, alice.clone(), keypair_alice, backend);

    // TODO transcript
    //     .add_evolvement(evolvement_add_in.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), client.get_members());

    let alice_after_insert = client
        .get_members()
        .into_iter()
        .find(|member| member.clone() == alice)
        .expect("Alice not found");

    let evolvement_remove_out = client
        .remove(&alice_after_insert, backend)
        .expect("failed to remove member");

    let evolvement_remove_in: <B::ClientProvider as EidClient>::EvolvementProvider =
        simulate_transfer(&evolvement_remove_out);
    client
        .evolve(evolvement_remove_in.clone(), backend)
        .expect("Failed to apply remove on client state");

    //TODO  transcript
    //     .add_evolvement(evolvement_remove_in.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), client.get_members());

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
fn update<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    let cross_sign_evolvement = cross_sign(client, backend);

    let alice_before_update_1 = &client.get_members()[0];

    let update_evolvement_1_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_1_in: <B::ClientProvider as EidClient>::EvolvementProvider =
        simulate_transfer(&update_evolvement_1_out);
    client
        .evolve(update_evolvement_1_in.clone(), backend)
        .expect("Failed to apply update on client state");
    // TODO transcript
    //     .add_evolvement(update_evolvement_1_in.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_1 = client.get_members();

    assert!(!members_after_update_1.contains(alice_before_update_1));
    assert_eq!(1, members_after_update_1.len());

    // Update Alice a second time
    let alice_before_update_2 = &members_after_update_1[0];
    let update_evolvement_2_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_2_in: <B::ClientProvider as EidClient>::EvolvementProvider =
        simulate_transfer(&update_evolvement_2_out);
    client
        .evolve(update_evolvement_2_in.clone(), backend)
        .expect("Failed to apply update on client state");

    // TODO transcript
    //     .add_evolvement(update_evolvement_2_in.clone(), backend)
    //     .expect("Failed to add evolvement");
    // assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_2 = client.get_members();

    assert!(!members_after_update_2.contains(alice_before_update_2));
    assert_eq!(1, members_after_update_2.len());
}

/// Create transcript, trusting the client's state
#[cfg(feature = "test")]
fn build_transcript<C>(client: &C, backend: &C::BackendProvider) -> C::TranscriptProvider
where
    C: EidClient,
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

fn cross_sign<C: EidClient>(client: &mut C, backend: &C::BackendProvider) -> C::EvolvementProvider {
    let cross_sign_evolvement_out = client
        .cross_sign_membership(backend)
        .expect("Cross signing failed");
    let cross_sign_evolvement_in: C::EvolvementProvider =
        simulate_transfer(&cross_sign_evolvement_out);

    client
        .evolve(cross_sign_evolvement_in.clone(), backend)
        .expect("Failed to apply state");

    cross_sign_evolvement_in
}

fn add_and_cross_sign<C: EidClient>(
    client: &mut C,
    member: C::MemberProvider,
    keypair: C::KeyProvider,
    backend: &C::BackendProvider,
) -> (C::EvolvementProvider, C::EvolvementProvider) {
    let add_evolvement_out = client.add(&member, backend).expect("failed to add member");
    let add_evolvement_in: C::EvolvementProvider = simulate_transfer(&add_evolvement_out);

    client
        .evolve(add_evolvement_in.clone(), backend)
        .expect("Failed to evolve");

    let new_client = &mut C::create_from_invitation(add_evolvement_in.clone(), keypair, backend)
        .expect("failed to create client from invitation");

    let cross_sign_evolvement_in = cross_sign(new_client, backend);

    client
        .evolve(cross_sign_evolvement_in.clone(), backend)
        .expect("Failed to evolve");

    (add_evolvement_in, cross_sign_evolvement_in)
}

#[test]
fn test_debug() {
    // let backend = &EidMlsBackend::default();
    let backend = &EidDummyBackend::default();
    update(backend);
}
