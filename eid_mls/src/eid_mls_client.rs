use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;
use openmls::prelude::{Ciphersuite, Extension, LifetimeExtension, OpenMlsCryptoProvider};

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_key_creation::{create_store_credential, create_store_key_package};
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

    fn create_eid(
        cred: <Self::MemberProvider as Member>::PubkeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        Self::create_mls_eid(backend, &cred)
    }

    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        let group = &mut self.state.group;
        let (mls_out, welcome) = group
            .add_members(&backend.mls_backend, &[member.get_pk()])
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
        let group = &mut self.state.group;

        // TODO: this will change massively with the new openmls version, as you have to supply a
        // node id and not a key package reference then
        let kp_ref = member
            .get_pk()
            .hash_ref(backend.mls_backend.crypto())
            .map_err(|error| EidError::RemoveMemberError(error.to_string()))?;
        let (mls_out, welcome) = group
            .remove_members(&backend.mls_backend, &[kp_ref])
            .map_err(|error| EidError::RemoveMemberError(error.to_string()))?;
        let evolvement = EidMlsEvolvement {
            message: mls_out,
            welcome,
        };
        Ok(evolvement)
    }

    fn update(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        let group = &mut self.state.group;
        let mls_out = group
            .propose_self_update(&backend.mls_backend, None)
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

    fn get_members(&self) -> Result<Vec<Self::MemberProvider>, EidError> {
        self.state.get_members()
    }

    fn export_transcript_state(&self) -> Self::TranscriptStateProvider {
        todo!()
    }

    fn generate_pubkey(
        backend: &Self::BackendProvider,
    ) -> <Self::MemberProvider as Member>::PubkeyProvider {
        let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519; // TODO: do we want to supply this as parameter as well?
        let identifier = String::from("id01"); // TODO: yeah, idk ...
        let credential_bundle = create_store_credential(
            identifier,
            &backend.mls_backend,
            ciphersuite.signature_algorithm(),
        );
        let extensions = vec![Extension::LifeTime(LifetimeExtension::new(
            60 * 60 * 24 * 90, // Maximum lifetime of 90 days, expressed in seconds
        ))];
        let key_bundle = create_store_key_package(
            ciphersuite,
            &credential_bundle,
            &backend.mls_backend,
            extensions,
        );

        // TODO: we're basically throwing away the private parts (but they're stored in the key store) - do we want this?
        key_bundle.key_package().clone()
    }
}
