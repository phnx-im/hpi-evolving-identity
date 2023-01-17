use mls_assist::group::Group as AssistedGroup;
use openmls::framing::MlsMessageInBody;
use openmls::key_packages::KeyPackage;
use openmls::prelude::{
    Ciphersuite, GroupInfo, MlsMessageIn, TlsDeserializeTrait, TlsSerializeTrait,
};

use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_key_creation::{create_store_credential, create_store_key_package};
use crate::eid_mls_member::EidMlsMember;
use crate::eid_mls_transcript::EidMlsTranscript;
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
    type InitialIdentityProvider = KeyPackage;

    #[cfg(feature = "test")]
    type TranscriptProvider = EidMlsTranscript;

    fn create_eid(
        identity: Self::InitialIdentityProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        Self::create_mls_eid(backend, &identity)
    }

    fn add(
        &mut self,
        identity: Self::InitialIdentityProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        let group = &mut self.state.group;
        let (mls_out, welcome) = group
            .add_members(&backend.mls_backend, &[identity])
            .map_err(|error| EidError::AddMemberError(error.to_string()))?;
        let evolvement = EidMlsEvolvement::OUT {
            message: mls_out.into(),
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

        let (mls_out, welcome) = group
            .remove_members(&backend.mls_backend, &[member.member.index])
            .map_err(|error| EidError::RemoveMemberError(error.to_string()))?;
        let evolvement = EidMlsEvolvement::OUT {
            message: mls_out.into(),
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
        let evolvement = EidMlsEvolvement::OUT {
            message: mls_out.into(),
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

    fn export_transcript_state(
        &self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::TranscriptStateProvider, EidError> {
        let mls_out = self
            .state
            .group
            .export_group_info(&backend.mls_backend, false)
            .map_err(|_| EidError::ExportGroupInfoError)?;
        let mls_out_bytes = mls_out
            .to_bytes()
            .map_err(|_| EidError::ExportGroupInfoError)?;
        let mls_in = MlsMessageIn::try_from_bytes(&mls_out_bytes)
            .map_err(|_| EidError::ExportGroupInfoError)?;
        if let MlsMessageInBody::GroupInfo(verifiable_group_info) = mls_in.extract() {
            let group_info = verifiable_group_info.into();
            let leaf_node = self
                .state
                .group
                .own_leaf()
                .ok_or(EidError::InvalidMemberError("Cannot export leaf".into()))?;

            let group = AssistedGroup::new(group_info, leaf_node.clone());

            Ok(EidMlsTranscriptState::new(group))
        } else {
            Err(EidError::ExportGroupInfoError)
        }
    }

    #[cfg(feature = "test")]
    fn generate_initial_id(backend: &Self::BackendProvider) -> Self::InitialIdentityProvider {
        let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519; // TODO: do we want to supply this as parameter as well?
        let identifier = String::from("id01"); // TODO: yeah, idk ...
        let credential_bundle = create_store_credential(
            identifier,
            &backend.mls_backend,
            ciphersuite.signature_algorithm(),
        );

        let key_bundle =
            create_store_key_package(ciphersuite, &credential_bundle, &backend.mls_backend);

        // TODO: we're basically throwing away the private parts (but they're stored in the key store) - do we want this?
        key_bundle.clone()
    }
}
