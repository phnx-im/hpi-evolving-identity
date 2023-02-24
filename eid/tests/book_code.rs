use std::fs::File;

use openmls::prelude::{AddMembersError, Ciphersuite, CommitToPendingProposalsError, CreateCommitError, Credential, CredentialType, CredentialWithKey, CryptoConfig, EmptyInputError, Extensions, GroupId, InnerState, JoinProposal, KeyPackage, LeafNodeIndex, Member, MlsGroup, MlsGroupConfig, OpenMlsCryptoProvider, ProcessedMessageContent, Proposal, RemoveMembersError, RemoveOperation, Sender, SignatureScheme};
use openmls::test_utils::bytes_to_hex;
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;

use eid_mls::eid_mls_backend::EidMlsBackend;
use eid_mls::eid_mls_client::EidMlsClient;
use eid_mls::eid_mls_evolvement::EidMlsEvolvement;
use eid_mls::eid_mls_member::EidMlsMember;
use eid_traits::backend::EidBackend;
use eid_traits::client::EidClient;
use eid_traits::member::Member as EidMember;
use helpers::helpers::simulate_transfer;

mod helpers;

fn create_backend() {
    // ANCHOR: create_backend
    use eid_mls::eid_mls_backend::EidMlsBackend;

    let backend = EidMlsBackend::default();
    // ANCHOR_END: create_backend

    // Suppress warning.
    let _backend = backend;
}


fn generate_credential(
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

fn generate_key_package(
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

/// This test simulates various group operations like Add, Update, Remove in a
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
#[test]
fn book_operations() {
    let backend = &EidMlsBackend::default();
    let (alice, alice_signature_keys) = EidMlsClient::generate_member("Alice".into(), backend);
    let (bob, bob_signature_keys) = EidMlsClient::generate_member("Bob".into(), backend);
    let (charlie, charlie_signature_keys) = EidMlsClient::generate_member("Charlie".into(), backend);
    let (dave, dave_signature_keys) = EidMlsClient::generate_member("Dave".into(), backend);


    // ANCHOR: alice_create_eid
    let mut alice_eid = EidMlsClient::create_eid(&alice, alice_signature_keys, backend)
        .expect("Could not create EID");
    // ANCHOR_END: alice_create_eid

    // === Alice adds Bob ===
    // ANCHOR: alice_adds_bob
    let add_bob_evolvement = alice_eid
        .add(&bob, backend)
        .expect("Could not add member.");
    // ANCHOR_END: alice_adds_bob

    let add_bob_evolvement: EidMlsEvolvement = simulate_transfer(&add_bob_evolvement);

    alice_eid
        .evolve(add_bob_evolvement.clone(), backend)
        .expect("Alice could not apply add_bob_evolvement");

    // ANCHOR: bob_joins_with_invitation
    let mut bob_eid = EidMlsClient::create_from_invitation(
        add_bob_evolvement,
        bob_signature_keys,
        backend,
    )
        .expect("Error joining EID from Invitation");
    // ANCHOR_END: bob_joins_with_invitation

    // ANCHOR: processing_evolvements
    let add_charlie_evolvement = bob_eid
        .add(&charlie, backend)
        .unwrap();

    let add_charlie_evolvement_in: EidMlsEvolvement = simulate_transfer(&add_charlie_evolvement);

    alice_eid
        .evolve(add_charlie_evolvement_in.clone(), backend)
        .expect("Alice could not apply add_charlie_evolvement");

    bob_eid
        .evolve(add_charlie_evolvement_in.clone(), backend)
        .expect("Bob could not apply add_charlie_evolvement");
    // ANCHOR_END: processing_evolvements

    let mut charlie_eid = EidMlsClient::create_from_invitation(
        add_charlie_evolvement_in,
        charlie_signature_keys,
        backend,
    )
        .expect("Error creating group from Welcome");

    let bob = charlie_eid
        .get_members()
        .into_iter()
        .find(|member| member.clone() == bob)
        .expect("Bob not found");

    // === Charlie removes Bob ===
    // ANCHOR: charlie_removes_bob
    let remove_bob_evolvement = charlie_eid
        .remove(&bob, backend)
        .expect("Could not remove Bob from Charlie's EID");
    // ANCHOR_END: charlie_removes_bob

    let remove_bob_evolvement: EidMlsEvolvement = simulate_transfer(&remove_bob_evolvement);

    alice_eid
        .evolve(remove_bob_evolvement.clone(), backend)
        .expect("Alice could not apply remove_bob_evolvement");
    bob_eid
        .evolve(remove_bob_evolvement.clone(), backend)
        .expect("Alice could not apply remove_bob_evolvement");
    charlie_eid
        .evolve(remove_bob_evolvement, backend)
        .expect("Alice could not apply remove_bob_evolvement");

    // ANCHOR: alice_update_self
    let alice_update_evolvement = alice_eid
        .update(backend)
        .expect("Alice could not update");
    // ANCHOR_END: alice_update_self


    let alice_update_evolvement: EidMlsEvolvement = simulate_transfer(&alice_update_evolvement);
    alice_eid
        .evolve(alice_update_evolvement.clone(), backend)
        .expect("Alice could not apply alice_update_evolvement");

    charlie_eid
        .evolve(alice_update_evolvement.clone(), backend)
        .expect("Charlie could not apply alice_update_evolvement");

    // === Alice removes Charlie and re-adds Bob ===
    /*
        // Create a new KeyPackageBundle for Bob
        let bob_key_package = generate_key_package(
            ciphersuite,
            bob_credential.clone(),
            Extensions::default(),
            backend,
            &bob_signature_keys,
        );

        // Create RemoveProposal and process it
        // ANCHOR: propose_remove
        let mls_message_out = alice_group
            .propose_remove_member(
                backend,
                &alice_signature_keys,
                charlie_group.own_leaf_index(),
            )
            .expect("Could not create proposal to remove Charlie.");
        // ANCHOR_END: propose_remove

        let charlie_processed_message = charlie_group
            .process_message(
                backend,
                mls_message_out
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Check that we received the correct proposals
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            charlie_processed_message.into_content()
        {
            if let Proposal::Remove(ref remove_proposal) = staged_proposal.proposal() {
                // Check that Charlie was removed
                assert_eq!(remove_proposal.removed(), charlie_group.own_leaf_index());
                // Store proposal
                charlie_group.store_pending_proposal(*staged_proposal.clone());
            } else {
                unreachable!("Expected a Proposal.");
            }

            // Check that Alice removed Charlie
            assert!(matches!(
                staged_proposal.sender(),
                Sender::Member(member) if *member == alice_group.own_leaf_index()
            ));
        } else {
            unreachable!("Expected a QueuedProposal.");
        }

        // Create AddProposal and process it
        // ANCHOR: propose_add
        let mls_message_out = alice_group
            .propose_add_member(backend, &alice_signature_keys, &bob_key_package)
            .expect("Could not create proposal to add Bob");
        // ANCHOR_END: propose_add

        let charlie_processed_message = charlie_group
            .process_message(
                backend,
                mls_message_out
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Check that we received the correct proposals
        // ANCHOR: inspect_add_proposal
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            charlie_processed_message.into_content()
        {
            // In the case we received an Add Proposal
            if let Proposal::Add(add_proposal) = staged_proposal.proposal() {
                // Check that Bob was added
                assert_eq!(
                    add_proposal.key_package().leaf_node().credential(),
                    &bob_credential.credential
                );
            } else {
                panic!("Expected an AddProposal.");
            }

            // Check that Alice added Bob
            assert!(matches!(
                staged_proposal.sender(),
                Sender::Member(member) if *member == alice_group.own_leaf_index()
            ));
            // Store proposal
            charlie_group.store_pending_proposal(*staged_proposal);
        }
        // ANCHOR_END: inspect_add_proposal
        else {
            unreachable!("Expected a QueuedProposal.");
        }

        // Commit to the proposals and process it
        let (queued_message, welcome_option, _group_info) = alice_group
            .commit_to_pending_proposals(backend, &alice_signature_keys)
            .expect("Could not flush proposals");

        let charlie_processed_message = charlie_group
            .process_message(
                backend,
                queued_message
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Merge Commit
        alice_group
            .merge_pending_commit(backend)
            .expect("error merging pending commit");

        // Merge Commit
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            charlie_processed_message.into_content()
        {
            charlie_group
                .merge_staged_commit(backend, *staged_commit)
                .expect("Error merging staged commit.");
        } else {
            unreachable!("Expected a StagedCommit.");
        }

        // Make sure the group contains two members
        assert_eq!(alice_group.members().count(), 2);

        // Check that Alice & Bob are the members of the group
        let members = alice_group.members().collect::<Vec<Member>>();
        assert_eq!(members[0].credential.identity(), b"Alice");
        assert_eq!(members[1].credential.identity(), b"Bob");

        // Bob creates a new group
        let mut bob_group = MlsGroup::new_from_welcome(
            backend,
            &mls_group_config,
            welcome_option
                .expect("Welcome was not returned")
                .into_welcome()
                .expect("Unexpected message type."),
            Some(alice_group.export_ratchet_tree()),
        )
            .expect("Error creating group from Welcome");

        // Make sure the group contains two members
        assert_eq!(alice_group.members().count(), 2);

        // Check that Alice & Bob are the members of the group
        let members = alice_group.members().collect::<Vec<Member>>();
        assert_eq!(members[0].credential.identity(), b"Alice");
        assert_eq!(members[1].credential.identity(), b"Bob");

        // Make sure the group contains two members
        assert_eq!(bob_group.members().count(), 2);

        // Check that Alice & Bob are the members of the group
        let members = bob_group.members().collect::<Vec<Member>>();
        assert_eq!(members[0].credential.identity(), b"Alice");
        assert_eq!(members[1].credential.identity(), b"Bob");

        // === Alice sends a message to the group ===
        let message_alice = b"Hi, I'm Alice!";
        let queued_message = alice_group
            .create_message(backend, &alice_signature_keys, message_alice)
            .expect("Error creating application message");

        let bob_processed_message = bob_group
            .process_message(
                backend,
                queued_message
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Get sender information
        // As provided by the processed message
        let sender_cred_from_msg = bob_processed_message.credential().clone();

        // As provided by looking up the sender manually via the `member()` function
        // ANCHOR: member_lookup
        let sender_cred_from_group =
            if let Sender::Member(sender_index) = bob_processed_message.sender() {
                bob_group
                    .member(*sender_index)
                    .expect("Could not find sender in group.")
                    .clone()
            } else {
                unreachable!("Expected sender type to be `Member`.")
            };
        // ANCHOR_END: member_lookup

        // Check that we received the correct message
        if let ProcessedMessageContent::ApplicationMessage(application_message) =
            bob_processed_message.into_content()
        {
            // Check the message
            assert_eq!(application_message.into_bytes(), message_alice);
            // Check that Alice sent the message
            assert_eq!(sender_cred_from_msg, sender_cred_from_group);
            assert_eq!(
                &sender_cred_from_msg,
                alice_group.credential().expect("Expected a credential.")
            );
        } else {
            unreachable!("Expected an ApplicationMessage.");
        }

        // === Bob leaves the group ===

        // ANCHOR: leaving
        let queued_message = bob_group
            .leave_group(backend, &bob_signature_keys)
            .expect("Could not leave group");
        // ANCHOR_END: leaving

        let alice_processed_message = alice_group
            .process_message(
                backend,
                queued_message
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Store proposal
        if let ProcessedMessageContent::ProposalMessage(staged_proposal) =
            alice_processed_message.into_content()
        {
            // Store proposal
            alice_group.store_pending_proposal(*staged_proposal);
        } else {
            unreachable!("Expected a QueuedProposal.");
        }

        // Should fail because you cannot remove yourself from a group
        assert_eq!(
            bob_group.commit_to_pending_proposals(backend, &bob_signature_keys),
            Err(CommitToPendingProposalsError::CreateCommitError(
                CreateCommitError::CannotRemoveSelf
            ))
        );

        let (queued_message, _welcome_option, _group_info) = alice_group
            .commit_to_pending_proposals(backend, &alice_signature_keys)
            .expect("Could not commit to proposals.");

        // Check that Bob's group is still active
        assert!(bob_group.is_active());

        // Check that we received the correct proposals
        if let Some(staged_commit) = alice_group.pending_commit() {
            let remove = staged_commit
                .remove_proposals()
                .next()
                .expect("Expected a proposal.");
            // Check that Bob was removed
            assert_eq!(
                remove.remove_proposal().removed(),
                bob_group.own_leaf_index()
            );
            // Check that Bob removed himself
            assert!(matches!(
                remove.sender(),
                Sender::Member(member) if *member == bob_group.own_leaf_index()
            ));
            // Merge staged Commit
        } else {
            unreachable!("Expected a StagedCommit.");
        }

        alice_group
            .merge_pending_commit(backend)
            .expect("Could not merge Commit.");

        let bob_processed_message = bob_group
            .process_message(
                backend,
                queued_message
                    .into_protocol_message()
                    .expect("Unexpected message type"),
            )
            .expect("Could not process message.");

        // Check that we received the correct proposals
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            bob_processed_message.into_content()
        {
            let remove = staged_commit
                .remove_proposals()
                .next()
                .expect("Expected a proposal.");
            // Check that Bob was removed
            assert_eq!(
                remove.remove_proposal().removed(),
                bob_group.own_leaf_index()
            );
            // Check that Bob removed himself
            assert!(matches!(
                remove.sender(),
                Sender::Member(member) if *member == bob_group.own_leaf_index()
            ));
            assert!(staged_commit.self_removed());
            // Merge staged Commit
            bob_group
                .merge_staged_commit(backend, *staged_commit)
                .expect("Error merging staged commit.");
        } else {
            unreachable!("Expected a StagedCommit.");
        }

        // Check that Bob's group is no longer active
        assert!(!bob_group.is_active());

        // Make sure the group contains one member
        assert_eq!(alice_group.members().count(), 1);

        // Check that Alice is the only member of the group
        let members = alice_group.members().collect::<Vec<Member>>();
        assert_eq!(members[0].credential.identity(), b"Alice");

        // === Re-Add Bob with external Add proposal ===

        // Create a new KeyPackageBundle for Bob
        let bob_key_package = generate_key_package(
            ciphersuite,
            bob_credential.clone(),
            Extensions::default(),
            backend,
            &bob_signature_keys,
        );

        // ANCHOR: external_join_proposal
        let proposal = JoinProposal::new(
            bob_key_package,
            alice_group.group_id().clone(),
            alice_group.epoch(),
            &bob_signature_keys,
        )
            .expect("Could not create external Add proposal");
        // ANCHOR_END: external_join_proposal

        // ANCHOR: decrypt_external_join_proposal
        let alice_processed_message = alice_group
            .process_message(
                backend,
                proposal
                    .into_protocol_message()
                    .expect("Unexpected message type."),
            )
            .expect("Could not process message.");
        match alice_processed_message.into_content() {
            ProcessedMessageContent::ExternalJoinProposalMessage(proposal) => {
                alice_group.store_pending_proposal(*proposal);
                let (_commit, welcome, _group_info) = alice_group
                    .commit_to_pending_proposals(backend, &alice_signature_keys)
                    .expect("Could not commit");
                assert_eq!(alice_group.members().count(), 1);
                alice_group
                    .merge_pending_commit(backend)
                    .expect("Could not merge commit");
                assert_eq!(alice_group.members().count(), 2);

                let bob_group = MlsGroup::new_from_welcome(
                    backend,
                    &mls_group_config,
                    welcome
                        .unwrap()
                        .into_welcome()
                        .expect("Unexpected message type."),
                    None,
                )
                    .expect("Bob could not join the group");
                assert_eq!(bob_group.members().count(), 2);
            }
            _ => unreachable!(),
        }
        // ANCHOR_END: decrypt_external_join_proposal
        // now cleanup
        alice_group
            .remove_members(backend, &alice_signature_keys, &[LeafNodeIndex::new(1)])
            .expect("Could not remove Bob");
        alice_group
            .merge_pending_commit(backend)
            .expect("Could not nerge commit");
        assert_eq!(alice_group.members().count(), 1);

        // === Save the group state ===

        // Create a new KeyPackageBundle for Bob
        let bob_key_package = generate_key_package(
            ciphersuite,
            bob_credential,
            Extensions::default(),
            backend,
            &bob_signature_keys,
        );

        // Add Bob to the group
        let (_queued_message, welcome, _group_info) = alice_group
            .add_members(backend, &alice_signature_keys, &[bob_key_package])
            .expect("Could not add Bob");

        // Merge Commit
        alice_group
            .merge_pending_commit(backend)
            .expect("error merging pending commit");

        let mut bob_group = MlsGroup::new_from_welcome(
            backend,
            &mls_group_config,
            welcome.into_welcome().expect("Unexpected message type."),
            Some(alice_group.export_ratchet_tree()),
        )
            .expect("Could not create group from Welcome");

        assert_eq!(
            alice_group.export_secret(backend, "before load", &[], 32),
            bob_group.export_secret(backend, "before load", &[], 32)
        );

        // Check that the state flag gets reset when saving
        assert_eq!(bob_group.state_changed(), InnerState::Changed);
        //save(&mut bob_group);

        let name = bytes_to_hex(
            bob_group
                .own_leaf_node()
                .unwrap()
                .signature_key()
                .as_slice(),
        )
            .to_lowercase();
        let path = TEMP_DIR
            .path()
            .join(format!("test_mls_group_{}.json", &name));
        let out_file = &mut File::create(path.clone()).expect("Could not create file");
        bob_group
            .save(out_file)
            .expect("Could not write group state to file");

        // Check that the state flag gets reset when saving
        assert_eq!(bob_group.state_changed(), InnerState::Persisted);

        let file = File::open(path).expect("Could not open file");
        let bob_group = MlsGroup::load(file).expect("Could not load group from file");

        // Make sure the state is still the same
        assert_eq!(
            alice_group.export_secret(backend, "after load", &[], 32),
            bob_group.export_secret(backend, "after load", &[], 32)
        );
    }

    #[apply(ciphersuites_and_backends)]
    fn test_empty_input_errors(ciphersuite: Ciphersuite, backend: &impl OpenMlsCryptoProvider) {
        let group_id = GroupId::from_slice(b"Test Group");

        // Generate credential bundles
        let (alice_credential, alice_signature_keys) = generate_credential(
            "Alice".into(),
            CredentialType::Basic,
            ciphersuite.signature_algorithm(),
            backend,
        );

        // Define the MlsGroup configuration
        let mls_group_config = MlsGroupConfig::test_default(ciphersuite);

        // === Alice creates a group ===
        let mut alice_group = MlsGroup::new_with_group_id(
            backend,
            &alice_signature_keys,
            &mls_group_config,
            group_id,
            alice_credential,
        )
            .expect("An unexpected error occurred.");

        assert_eq!(
            alice_group
                .add_members(backend, &alice_signature_keys, &[])
                .expect_err("No EmptyInputError when trying to pass an empty slice to `add_members`."),
            AddMembersError::EmptyInput(EmptyInputError::AddMembers)
        );
        assert_eq!(
            alice_group
                .remove_members(backend, &alice_signature_keys, &[])
                .expect_err(
                    "No EmptyInputError when trying to pass an empty slice to `remove_members`."
                ),
            RemoveMembersError::EmptyInput(EmptyInputError::RemoveMembers)
        );*/
}
