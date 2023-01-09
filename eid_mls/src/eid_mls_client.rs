use openmls::error;
use openmls::prelude::{Ciphersuite, KeyPackage, MlsMessageIn, ProcessedMessage};

use eid_traits::client::EidClient;
use eid_traits::evolvement;
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
            .map_err(|error| EidError::AddMemberError(error.to_string()))?;
        let evolvement = EidMlsEvolvement {
            message: mls_out,
            welcome: Some(welcome),
        };
        Ok(evolvement)
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
        let group = &mut self.state.group;
        let mls_out = group
            .propose_self_update(backend.mls_backend, None)
            .map_err(|error| EidError::UpdateMemberError(error.to_string()))?;
        let evolvement = EidMlsEvolvement {
            message: mls_out,
            welcome: None,
        };
        Ok(evolvement)
    }

    fn evolve(
        &mut self,
        evolvement: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError> {
        Ok(self.state.apply(evolvement, backend)?)
    }
}
