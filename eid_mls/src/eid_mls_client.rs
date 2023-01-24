use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_key_creation::{create_store_credential, create_store_key_package};
use crate::eid_mls_member::EidMlsMember;
use crate::eid_mls_transcript::EidMlsTranscript;
use crate::state::client_state::EidMlsClientState;
use crate::state::transcript_state::{EidMlsExportedTranscriptState, EidMlsTranscriptState};

pub struct EidMlsClient {
    pub(crate) state: EidMlsClientState,
}

impl EidClient for EidMlsClient {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type TranscriptStateProvider = EidMlsTranscriptState;
    type ExportedTranscriptStateProvider = EidMlsExportedTranscriptState;
    type BackendProvider = EidMlsBackend;

    #[cfg(feature = "test")]
    type TranscriptProvider = EidMlsTranscript;

    fn create_eid(
        initial_member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        Self::create_mls_eid(backend, &initial_member)
    }

    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        let group = &mut self.state.group;
        let (mls_out, welcome, _group_info) = group
            .add_members(&backend.mls_backend, &[member.key_package.clone()])
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

        if let Some(mls_member) = &member.mls_member {
            let (mls_out, welcome, _group_info) = group
                .remove_members(&backend.mls_backend, &[mls_member.index])
                .map_err(|error| EidError::RemoveMemberError(error.to_string()))?;
            let evolvement = EidMlsEvolvement::OUT {
                message: mls_out.into(),
                welcome,
            };
            Ok(evolvement)
        } else {
            Err(EidError::InvalidMemberError(
                "Member not in MLS Group".into(),
            ))
        }
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

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.state.get_members()
    }

    fn export_transcript_state(
        &self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::ExportedTranscriptStateProvider, EidError> {
        let mls_out = self
            .state
            .group
            .export_group_info(&backend.mls_backend, false)
            .map_err(|_| EidError::ExportGroupInfoError)?;
        let leaf_node = self
            .state
            .group
            .own_leaf()
            .ok_or(EidError::InvalidMemberError("Cannot export leaf".into()))?
            .clone();

        Ok(EidMlsExportedTranscriptState::OUT {
            group_info: mls_out,
            leaf_node,
        })
    }

    #[cfg(feature = "test")]
    fn generate_initial_id(id: String, backend: &Self::BackendProvider) -> Self::MemberProvider {
        let ciphersuite = backend.ciphersuite;
        let credential_bundle =
            create_store_credential(id, &backend.mls_backend, ciphersuite.signature_algorithm());

        let key_bundle =
            create_store_key_package(ciphersuite, &credential_bundle, &backend.mls_backend);

        // TODO: we're basically throwing away the private parts (but they're stored in the key store) - do we want this?
        EidMlsMember {
            mls_member: None,
            key_package: key_bundle.clone(),
        }
    }
}
