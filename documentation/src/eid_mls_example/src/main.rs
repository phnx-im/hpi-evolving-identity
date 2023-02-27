use openmls::prelude::CredentialType;
use openmls_basic_credential::SignatureKeyPair;

use eid::test_helpers::simulate_transfer;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_mls::eid_mls_evolvement::EidMlsEvolvement;
use eid_mls::eid_mls_member::EidMlsMember;
use eid_mls::eid_mls_transcript::EidMlsTranscript;
use eid_mls::state::transcript_state::EidMlsExportedTranscriptState;
use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::transcript::{EidExportedTranscriptState, EidTranscript};

pub fn create_backend() {
    #[allow(unused_imports)]
    // ANCHOR: create_backend
    use eid_mls::eid_mls_backend::EidMlsBackend;
    let backend = EidMlsBackend::default();
    // ANCHOR_END: create_backend

    // Suppress warning.
    let _backend = backend;
}

pub fn generate_member(
    identity: Vec<u8>,
    backend: &EidMlsBackend,
) -> (EidMlsMember, SignatureKeyPair) {
    // ANCHOR: create_member
    let ciphersuite = backend.ciphersuite;
    let (cred_with_key, keypair) = EidMlsClient::create_store_credential(
        identity,
        CredentialType::Basic,
        ciphersuite.signature_algorithm(),
        backend,
    )
    .expect("Failed to create credential");

    let key_package = EidMlsClient::create_store_key_package(
        ciphersuite,
        cred_with_key.clone(),
        backend,
        &keypair,
    )
    .expect("Failed to create key package");

    let member = EidMlsMember::new((key_package, cred_with_key));
    // ANCHOR_END: create_member
    (member, keypair)
}

/// This function simulates various group operations like Add, Update, Remove in a
/// small group
///  - Alice creates an EID
///  - Alice adds Bob
///  - Alice updates key material
///  - Bob removes Alice
fn book_operations() {
    // ANCHOR: full_example
    let backend = &EidMlsBackend::default();
    let (alice, alice_signature_keys) = generate_member("Alice".into(), backend);
    let (bob, bob_signature_keys) = generate_member("Bob".into(), backend);

    // ANCHOR: alice_create_eid
    let mut alice_client = EidMlsClient::create_eid(&alice, alice_signature_keys, backend).unwrap();
    // ANCHOR_END: alice_create_eid

    // ANCHOR: create_transcript
    let exported_state = alice_client.export_transcript_state(backend).unwrap();

    // exported state is sent over the wire
    let imported_state: EidMlsExportedTranscriptState = simulate_transfer(&exported_state);

    // validate the state and create a trusted state from it
    let trusted_state = imported_state.into_transcript_state(backend).unwrap();
    let mut transcript = EidMlsTranscript::new(trusted_state, vec![], backend).unwrap();
    // ANCHOR_END: create_transcript

    assert_eq!(transcript.get_members().len(), 0);
    assert_eq!(alice_client.get_members().len(), 0);
    // === Alice adds Bob ===

    let alice_cross_sign_evolvement_out = alice_client.cross_sign_membership(backend).unwrap();
    let alice_cross_sign_evolvement_in: EidMlsEvolvement =
        simulate_transfer(&alice_cross_sign_evolvement_out);

    transcript
        .evolve(alice_cross_sign_evolvement_in.clone(), backend)
        .unwrap();
    alice_client
        .evolve(alice_cross_sign_evolvement_in, backend)
        .unwrap();
    assert_eq!(transcript.get_members().len(), 1);
    assert_eq!(alice_client.get_members().len(), 1);

    // ANCHOR: alice_adds_bob
    let add_bob_evolvement_out = alice_client.add(&bob, backend).unwrap();
    let add_bob_evolvement_in: EidMlsEvolvement = simulate_transfer(&add_bob_evolvement_out);
    transcript
        .evolve(add_bob_evolvement_in.clone(), backend)
        .unwrap();
    alice_client
        .evolve(add_bob_evolvement_in.clone(), backend)
        .unwrap();
    // ANCHOR_END: alice_adds_bob

    // ANCHOR: bob_joins_with_invitation
    let mut bob_client =
        EidMlsClient::create_from_invitation(add_bob_evolvement_in, bob_signature_keys, backend)
            .unwrap();

    // Bob hasn't cross signed yet, member list is unchanged
    assert_eq!(transcript.get_members().len(), 1);
    assert_eq!(alice_client.get_members().len(), 1);
    assert_eq!(bob_client.get_members().len(), 1);

    let add_cross_sign_evolvement_bob_out = bob_client.cross_sign_membership(backend).unwrap();

    let add_cross_sign_evolvement_bob_in: EidMlsEvolvement =
        simulate_transfer(&add_cross_sign_evolvement_bob_out);
    transcript
        .evolve(add_cross_sign_evolvement_bob_in.clone(), backend)
        .unwrap();
    alice_client
        .evolve(add_cross_sign_evolvement_bob_in.clone(), backend)
        .unwrap();
    bob_client
        .evolve(add_cross_sign_evolvement_bob_in, backend)
        .unwrap();

    // After cross sign, Bob is part of the EID
    assert_eq!(transcript.get_members().len(), 2);
    assert_eq!(alice_client.get_members().len(), 2);
    assert_eq!(bob_client.get_members().len(), 2);

    // ANCHOR_END: bob_joins_with_invitation

    // ANCHOR: alice_update_self
    let alice_update_evolvement_out = alice_client.update(backend).unwrap();

    let alice_update_evolvement_in: EidMlsEvolvement =
        simulate_transfer(&alice_update_evolvement_out);
    transcript
        .evolve(alice_update_evolvement_in.clone(), backend)
        .unwrap();
    alice_client
        .evolve(alice_update_evolvement_in.clone(), backend)
        .unwrap();
    bob_client
        .evolve(alice_update_evolvement_in, backend)
        .unwrap();
    // ANCHOR_END: alice_update_self
    assert_eq!(transcript.get_members().len(), 2);
    assert_eq!(alice_client.get_members().len(), 2);
    assert_eq!(bob_client.get_members().len(), 2);

    // ANCHOR: bob_removes_alice

    // Bob finds alice in his member list
    let alice = bob_client
        .get_members()
        .into_iter()
        .find(|member| member.clone() == alice)
        .unwrap();

    let remove_alice_evolvement_out = bob_client.remove(&alice, backend).unwrap();

    let remove_alice_evolvement_in: EidMlsEvolvement =
        simulate_transfer(&remove_alice_evolvement_out);

    transcript
        .evolve(remove_alice_evolvement_in.clone(), backend)
        .unwrap();

    bob_client
        .evolve(remove_alice_evolvement_in, backend)
        .unwrap();

    assert_eq!(transcript.get_members().len(), 1);
    assert_eq!(bob_client.get_members().len(), 1);

    // ANCHOR_END: bob_removes_alice
    // ANCHOR_END: full_example
}

fn main() {
    book_operations()
}
