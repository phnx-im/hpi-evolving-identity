use openmls::prelude::{Ciphersuite, MlsMessageIn, ProcessedMessage};
use openmls_rust_crypto::OpenMlsRustCrypto;

use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::types::EidError;

use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::client_state::EidMlsClientState;

pub struct EidMlsClient {
    pub(crate) state: EidMlsClientState,
}

impl EidClient for EidMlsClient {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type StateProvider = EidMlsClientState;
    type BackendProvider = OpenMlsRustCrypto;

    fn state(&self) -> &Self::StateProvider {
        todo!()
    }

    fn pk(&self) -> &[u8] {
        todo!()
    }

    fn generate_credential(
        backend: &Self::BackendProvider,
    ) -> <Self::MemberProvider as Member>::CredentialProvider {
        todo!()
    }

    fn create_eid(backend: &Self::BackendProvider) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519; // TODO: do we want to supply this as parameter as well?

        Self::create_mls_eid(backend, ciphersuite)
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
            .add_members(self.backend, &[member.get_key_package()])
            .expect("Could not add member");
        let mls_in: MlsMessageIn = mls_out.into();
        let unverified_msg = group
            .parse_message(mls_in.clone(), backend)
            .expect("Could not parse message");
        let proc_msg = group
            .process_unverified_message(unverified_msg, None, backend)
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
