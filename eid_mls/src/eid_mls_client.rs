use openmls::prelude::{Ciphersuite, KeyPackage, MlsMessageIn, ProcessedMessage};

use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::client_state::EidMlsClientState;
use crate::state::transcript_state::EidMlsTranscriptState;

pub struct EidMlsClient {
    pub(crate) state: EidMlsClientState,
}

impl EidClient for EidMlsClient {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type TranscriptStateProvider = EidMlsTranscriptState;
    type BackendProvider = EidMlsBackend;

    fn export_transcript_state(&self) -> Self::TranscriptStateProvider {
        todo!()
    }

    fn generate_credential(
        backend: &Self::BackendProvider,
    ) -> <Self::MemberProvider as Member>::CredentialProvider {
        todo!()
    }

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
        self.state.get_members()
    }

    fn get_credential(&self) -> &KeyPackage {
        todo!()
    }

    fn create_eid(
        cred: <Self::MemberProvider as Member>::CredentialProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519; // TODO: do we want to supply this as parameter as well?
        Self::create_mls_eid(backend, ciphersuite) // TODO: use cred
    }

    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized,
    {
        let group = &mut self.state.group;
        let (mls_out, welcome) = group
            .add_members(backend.mls_backend, &[member.get_key_package()])
            .expect("Could not add member");
        let mls_in: MlsMessageIn = mls_out.into();
        let unverified_msg = group
            .parse_message(mls_in.clone(), backend.mls_backend)
            .expect("Could not parse message");
        let proc_msg = group
            .process_unverified_message(unverified_msg, None, backend.mls_backend)
            .expect("Can't process message");
        return if let ProcessedMessage::StagedCommitMessage(staged_commit) = proc_msg {
            Ok(EidMlsEvolvement {
                commit: *staged_commit,
                message: mls_in,
            })
        } else {
            Err(EidError::ParseMessageError)
        };
    }

    fn remove(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        todo!()
    }

    fn evolve(
        &mut self,
        evolvement: &Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        todo!()
    }
}
