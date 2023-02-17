#[cfg(feature = "test")]
pub mod helpers {
    use tls_codec::{Deserialize, Serialize};

    use eid_traits::client::EidClient;
    use eid_traits::transcript::EidTranscript;

    /// Simulate transfer over the wire by simply serializing and deserializing once.
    pub fn simulate_transfer<I: Serialize, O: Deserialize>(input: &I) -> O {
        let serialized = input.tls_serialize_detached().expect("Failed to serialize");
        O::tls_deserialize(&mut serialized.as_slice()).expect("Failed to deserialize")
    }

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
