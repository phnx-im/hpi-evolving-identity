use openmls::prelude::{
    Ciphersuite, Credential, CredentialType, CredentialWithKey, CryptoConfig, Extensions,
    KeyPackage, OpenMlsCryptoProvider, SignatureScheme,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;

use eid::test_helpers::simulate_transfer;
use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_mls::eid_mls_evolvement::EidMlsEvolvement;
use eid_traits::client::EidClient;

pub fn create_backend() {
    // ANCHOR: create_backend
    use eid_mls::eid_mls_backend::EidMlsBackend;
    let backend = EidMlsBackend::default();
    // ANCHOR_END: create_backend

    // Suppress warning.
    let _backend = backend;
}

pub fn generate_credential(
    identity: Vec<u8>,
    credential_type: CredentialType,
    signature_algorithm: SignatureScheme,
    backend: &impl OpenMlsCryptoProvider,
) -> (CredentialWithKey, SignatureKeyPair) {
    // ANCHOR: create_basic_credential
    let credential = Credential::new(identity, credential_type).unwrap();
    // ANCHOR_END: create_basic_credential
    // ANCHOR: create_credential_keys
    let signature_keys = SignatureKeyPair::new(signature_algorithm).unwrap();
    signature_keys.store(backend.key_store()).unwrap();
    // ANCHOR_END: create_credential_keys

    (
        CredentialWithKey {
            credential,
            signature_key: signature_keys.to_public_vec().into(),
        },
        signature_keys,
    )
}

pub fn generate_key_package(
    ciphersuite: Ciphersuite,
    credential_with_key: CredentialWithKey,
    extensions: Extensions,
    backend: &impl OpenMlsCryptoProvider,
    signer: &impl Signer,
) -> KeyPackage {
    // ANCHOR: create_key_package
    // Create the key package
    KeyPackage::builder()
        .key_package_extensions(extensions)
        .build(
            CryptoConfig::with_default_version(ciphersuite),
            backend,
            signer,
            credential_with_key,
        )
        .unwrap()
    // ANCHOR_END: create_key_package
}

/// This function simulates various group operations like Add, Update, Remove in a
/// small group
///  - Alice creates a group
///  - Alice adds Bob
///  - Alice sends a message to Bob
///  - Bob updates and commits
///  - Alice updates and commits
///  - Bob adds Charlie
///  - Charlie sends a message to the group
///  - Charlie updates and commits
///  - Charlie removes Bob
///  - Alice removes Charlie and adds Bob
///  - Bob leaves
///  - Test saving the group state
fn book_operations() {
    let backend = &EidMlsBackend::default();
    let (alice, alice_signature_keys) = EidMlsClient::generate_member("Alice".into(), backend);
    let (bob, bob_signature_keys) = EidMlsClient::generate_member("Bob".into(), backend);
    let (charlie, charlie_signature_keys) =
        EidMlsClient::generate_member("Charlie".into(), backend);

    // ANCHOR: alice_create_eid
    let mut alice_client = EidMlsClient::create_eid(&alice, alice_signature_keys, backend)
        .expect("Could not create EID");
    // ANCHOR_END: alice_create_eid

    assert_eq!(alice_client.get_members().len(), 0);
    // === Alice adds Bob ===
    // ANCHOR: alice_adds_bob
    let add_bob_evolvement = alice_client
        .add(&bob, backend)
        .expect("Could not add member.");
    // ANCHOR_END: alice_adds_bob

    let add_bob_evolvement: EidMlsEvolvement = simulate_transfer(&add_bob_evolvement);

    alice_client
        .evolve(add_bob_evolvement.clone(), backend)
        .expect("Alice could not apply add_bob_evolvement");
    let members = alice_client.get_members();
    assert_eq!(members.len(), 1);

    // ANCHOR: bob_joins_with_invitation
    let mut bob_client =
        EidMlsClient::create_from_invitation(add_bob_evolvement, bob_signature_keys, backend)
            .expect("Error joining EID from Invitation");
    // ANCHOR_END: bob_joins_with_invitation

    // ANCHOR: processing_evolvements
    let add_charlie_evolvement = bob_client.add(&charlie, backend).unwrap();
    assert_eq!(members.len(), 1);

    let add_charlie_evolvement_in: EidMlsEvolvement = simulate_transfer(&add_charlie_evolvement);

    alice_client
        .evolve(add_charlie_evolvement_in.clone(), backend)
        .expect("Alice could not apply add_charlie_evolvement");
    assert_eq!(alice_client.get_members().len(), 2);

    bob_client
        .evolve(add_charlie_evolvement_in.clone(), backend)
        .expect("Bob could not apply add_charlie_evolvement");
    // ANCHOR_END: processing_evolvements
    assert_eq!(bob_client.get_members().len(), 2);

    let mut charlie_client = EidMlsClient::create_from_invitation(
        add_charlie_evolvement_in,
        charlie_signature_keys,
        backend,
    )
    .expect("Error creating group from Welcome");

    let bob = charlie_client
        .get_members()
        .into_iter()
        .find(|member| member.clone() == bob)
        .expect("Bob not found");

    // === Charlie removes Bob ===
    // ANCHOR: charlie_removes_bob
    let remove_bob_evolvement = charlie_client
        .remove(&bob, backend)
        .expect("Could not remove Bob from Charlie's EID");
    // ANCHOR_END: charlie_removes_bob

    let remove_bob_evolvement: EidMlsEvolvement = simulate_transfer(&remove_bob_evolvement);

    alice_client
        .evolve(remove_bob_evolvement.clone(), backend)
        .expect("Alice could not apply remove_bob_evolvement");
    assert_eq!(alice_client.get_members().len(), 1);

    bob_client
        .evolve(remove_bob_evolvement.clone(), backend)
        .expect("Alice could not apply remove_bob_evolvement");
    assert_eq!(bob_client.get_members().len(), 1);

    charlie_client
        .evolve(remove_bob_evolvement, backend)
        .expect("Alice could not apply remove_bob_evolvement");
    assert_eq!(charlie_client.get_members().len(), 2);

    // ANCHOR: alice_update_self
    let alice_update_evolvement = alice_client
        .update(backend)
        .expect("Alice could not update");
    // ANCHOR_END: alice_update_self

    let alice_update_evolvement: EidMlsEvolvement = simulate_transfer(&alice_update_evolvement);
    alice_client
        .evolve(alice_update_evolvement.clone(), backend)
        .expect("Alice could not apply alice_update_evolvement");

    charlie_client
        .evolve(alice_update_evolvement.clone(), backend)
        .expect("Charlie could not apply alice_update_evolvement");
}

fn main() {
    book_operations()
}
