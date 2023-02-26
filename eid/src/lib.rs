/// # Test Helpers
/// This module contains convenience methods that are used for tests.
#[cfg(feature = "test")]
pub mod test_helpers {
    use tls_codec::{Deserialize, Serialize};

    use eid_traits::client::EidClient;
    use eid_traits::transcript::{EidExportedTranscriptState, EidTranscript};

    /// Build a transcript by exporting the client's state and creating a transcript from it.
    /// This state will be used as the trusted state for the transcript.
    ///
    /// # Arguments
    ///
    /// * `client`: The client
    /// * `backend`: The EID backend
    ///
    /// returns: The [EidTranscript].
    pub fn build_transcript<C>(client: &C, backend: &C::BackendProvider) -> C::TranscriptProvider
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

    /// "Simulate" a transfer over the wire by serializing and deserializing the given item.
    ///
    /// # Arguments
    ///
    /// * `input`: The item that is serialized.
    ///
    /// returns: The deserialized struct.
    pub fn simulate_transfer<I: Serialize, O: Deserialize>(input: &I) -> O {
        let serialized = input.tls_serialize_detached().expect("Failed to serialize");
        O::tls_deserialize(&mut serialized.as_slice()).expect("Failed to deserialize")
    }

    /// Cross-sign the client's EID membership and evolve client and transcript
    ///
    /// # Arguments
    ///
    /// * `client`: The client
    /// * `transcript`: The transcript
    /// * `backend`: The EID backend
    ///
    /// returns: [EidClient::EvolvementProvider] The transferred cross sign evolvement
    pub fn cross_sign<C: EidClient>(
        client: &mut C,
        transcript: &mut C::TranscriptProvider,
        backend: &C::BackendProvider,
    ) -> C::EvolvementProvider {
        let cross_sign_evolvement_out = client
            .cross_sign_membership(backend)
            .expect("Cross signing failed");
        let cross_sign_evolvement_in: C::EvolvementProvider =
            simulate_transfer(&cross_sign_evolvement_out);

        transcript
            .evolve(cross_sign_evolvement_in.clone(), backend)
            .expect("Failed to add cross sign evolvement to transcript");
        client
            .evolve(cross_sign_evolvement_in.clone(), backend)
            .expect("Failed to apply state");
        cross_sign_evolvement_in
    }

    /// Add a member to the Eid and evolve [EidClient] and [EidTranscript].
    /// Then let the new_members client cross-sign and evolve client and transcript.
    ///
    /// # Arguments
    ///
    /// * `client`: The client
    /// * `transcript`: The transcript
    /// * `member`: The member
    /// * `keypair`: The member's key material
    /// * `backend`: The EID backend
    ///
    /// returns: ([EidClient::EvolvementProvider], [EidClient::EvolvementProvider])
    /// The transferred evolvements of the addition and cross-signing.
    pub fn add_and_cross_sign<C: EidClient>(
        client: &mut C,
        transcript: &mut C::TranscriptProvider,
        member: C::MemberProvider,
        keypair: C::KeyProvider,
        backend: &C::BackendProvider,
    ) -> (C::EvolvementProvider, C::EvolvementProvider) {
        let add_evolvement_out = client.add(&member, backend).expect("failed to add member");
        let add_evolvement_in: C::EvolvementProvider = simulate_transfer(&add_evolvement_out);

        transcript
            .evolve(add_evolvement_in.clone(), backend)
            .expect("Failed to add evolvement to transcript");

        client
            .evolve(add_evolvement_in.clone(), backend)
            .expect("Failed to evolve");

        let new_client =
            &mut C::create_from_invitation(add_evolvement_in.clone(), keypair, backend)
                .expect("failed to create client from invitation");

        let cross_sign_evolvement_in = cross_sign(new_client, transcript, backend);

        client
            .evolve(cross_sign_evolvement_in.clone(), backend)
            .expect("Failed to evolve");
        (add_evolvement_in, cross_sign_evolvement_in)
    }
}
