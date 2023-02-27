pub use rstest::*;
pub use rstest_reuse::{self, *};

use eid_dummy::eid_dummy_backend::EidDummyBackend;
pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_traits::backend::EidBackend;
use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::transcript::EidTranscript;
use eid_traits::types::EidError;
use helpers::helpers::{add_and_cross_sign, build_transcript, cross_sign, simulate_transfer};

pub mod helpers;

#[template]
#[rstest(backend,
case::EidDummy(& EidDummyBackend::default()),
case::EidMls(& EidMlsBackend::default()),
)]
#[allow(non_snake_case)]
pub fn eid_backends<B: EidBackend>(backend: &B) {}

/// This test simulates the following:
/// * create a new client
/// * create a transcript from the client's state
/// * let the client cross sign its membership
/// * create and add a second member to the EID
/// * let the second member cross sign its membership
/// * add the same member a second time, expecting an Error
///
/// We evolve client and transcript along the way, comparing their states
///
#[apply(eid_backends)]
fn add<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    // member list length unchanged before evolving
    let members_initial = client.get_members();

    assert_eq!(0, members_initial.len());

    cross_sign(client, &mut transcript, backend);

    let members_after_cross_sign = client.get_members();
    assert_eq!(1, members_after_cross_sign.len());

    // Create Alice as a member with a random pk
    let (alice, alice_kp) = B::ClientProvider::generate_member("alice".into(), backend);
    let (add_alice_evolvement_in, ..) =
        add_and_cross_sign(client, &mut transcript, alice.clone(), alice_kp, backend);

    assert_eq!(client.get_members().len(), 2);
    let error = client
        .evolve(add_alice_evolvement_in.clone(), backend)
        .expect_err("Evolving with same evolvement twice");
    assert!(matches!(error, EidError::InvalidEvolvementError(..)));

    let error = transcript
        .evolve(add_alice_evolvement_in.clone(), backend)
        .expect_err("Evolving with same evolvement twice");
    assert!(matches!(error, EidError::InvalidEvolvementError(..)));

    let members_after_alice_cross_sign = client.get_members();
    assert!(members_after_alice_cross_sign.contains(&alice));
    assert_eq!(2, members_after_alice_cross_sign.len());

    assert_eq!(transcript.get_members(), members_after_alice_cross_sign);

    // Try to add Alice a second time
    let member_in_eid_error = client
        .add(&alice, backend)
        .expect_err("Adding member a second time");
    assert!(matches!(member_in_eid_error, EidError::AddMemberError(..)));

    // Add Bob
    let (bob, bob_kp) = B::ClientProvider::generate_member("bob".into(), backend);
    add_and_cross_sign(client, &mut transcript, bob.clone(), bob_kp, backend);

    let members = client.get_members();
    assert!(members.contains(&bob));
    assert_eq!(3, members.len());

    assert_eq!(transcript.get_members(), members);
}

/// This test simulates the following:
/// * create a new client
/// * create a transcript from the client's state
/// * let the client cross sign its membership
/// * create and add a second member to the EID
/// * remove the second member
/// * remove the same member a second time, expecting an Error
///
/// We evolve client and transcript along the way, comparing their states
///
#[apply(eid_backends)]
fn remove<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    cross_sign(client, &mut transcript, backend);

    let (alice, keypair_alice) = B::ClientProvider::generate_member("alice".into(), backend);
    add_and_cross_sign(
        client,
        &mut transcript,
        alice.clone(),
        keypair_alice,
        backend,
    );

    assert_eq!(transcript.get_members(), client.get_members());

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
    transcript
        .evolve(evolvement_remove_in.clone(), backend)
        .expect("Failed to evolve transcript");

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

/// This test simulates the following:
/// * create a new client
/// * create a transcript from the client's state
/// * let the client cross sign its membership
/// * let the client update its key material twice
///
/// We evolve client and transcript along the way, comparing their states
#[apply(eid_backends)]
fn update<B: EidBackend>(backend: &B) {
    let client = &mut B::ClientProvider::generate_initial_client("test_id".into(), backend);
    let mut transcript = build_transcript(client, backend);

    cross_sign(client, &mut transcript, backend);

    let alice_before_update_1 = &client.get_members()[0];

    let update_evolvement_1_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_1_in: <B::ClientProvider as EidClient>::EvolvementProvider =
        simulate_transfer(&update_evolvement_1_out);
    client
        .evolve(update_evolvement_1_in.clone(), backend)
        .expect("Failed to apply update on client state");
    transcript
        .evolve(update_evolvement_1_in.clone(), backend)
        .expect("Failed to evolve transcript");

    assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_1 = client.get_members();

    let alice_after_update = members_after_update_1[0].clone();
    assert!(members_after_update_1.contains(alice_before_update_1));
    assert_ne!(alice_after_update.get_pk(), alice_before_update_1.get_pk());
    assert_eq!(1, members_after_update_1.len());

    // Update Alice a second time
    let alice_before_update_2 = &members_after_update_1[0];
    let update_evolvement_2_out = client.update(backend).expect("Updating client keys failed");
    let update_evolvement_2_in: <B::ClientProvider as EidClient>::EvolvementProvider =
        simulate_transfer(&update_evolvement_2_out);
    client
        .evolve(update_evolvement_2_in.clone(), backend)
        .expect("Failed to apply update on client state");

    transcript
        .evolve(update_evolvement_2_in.clone(), backend)
        .expect("Failed to add evolvement");
    assert_eq!(transcript.get_members(), client.get_members());

    let members_after_update_2 = client.get_members();
    let alice_after_update_2 = members_after_update_2[0].clone();

    assert!(members_after_update_2.contains(alice_before_update_2));
    assert_ne!(
        alice_before_update_2.get_pk(),
        alice_after_update_2.get_pk()
    );
    assert_eq!(1, members_after_update_2.len());
}
