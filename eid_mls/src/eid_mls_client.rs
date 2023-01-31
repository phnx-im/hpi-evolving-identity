use openmls::framing::MlsMessageIn;
use openmls::messages::Welcome;
use openmls::prelude::MlsMessageInBody;
use openmls::prelude::{CredentialType, MlsGroup, SignaturePublicKey};
use openmls_basic_credential::SignatureKeyPair;

use eid_traits::client::EidClient;
use eid_traits::evolvement::Evolvement;
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
    pub(crate) keypair: SignatureKeyPair,
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
        key_pair: Self::KeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let key_package =
            initial_member
                .key_package
                .clone()
                .ok_or(EidError::InvalidMemberError(
                    "No key package provided in member".into(),
                ))?;

        let signature_key = key_package.leaf_node().signature_key();

        Self::create_mls_eid(backend, key_pair, initial_member.credential.clone())
    }

    fn create_from_invitation(
        invitation: Self::EvolvementProvider,
        signature_keypair: Self::KeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        if let EidMlsEvolvement::IN {
            message: _,
            welcome: option_message_in,
        } = invitation
        {
            let message_in = option_message_in.ok_or(EidError::InvalidInvitationError)?;
            let message_in_body = message_in.extract();
            if let MlsMessageInBody::Welcome(welcome) = message_in_body {
                let mls_group_config = Self::gen_group_config();
                let mls_group = MlsGroup::new_from_welcome(
                    &backend.mls_backend,
                    &mls_group_config,
                    welcome,
                    None,
                )
                .map_err(|err| EidError::CreateGroupError(err.to_string()))?;
                return Ok(Self {
                    state: EidMlsClientState { group: mls_group },
                    keypair: signature_keypair,
                });
            }
        }
        Err(EidError::InvalidInvitationError)
    }

    fn add(
        &mut self,
        member: &Self::MemberProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        if let Some(key_package) = member.key_package.clone() {
            let group = &mut self.state.group;
            let (mls_out, welcome, _group_info) = group
                .add_members(&backend.mls_backend, &self.pubkey, &[key_package])
                .map_err(|error| EidError::AddMemberError(error.to_string()))?;
            let evolvement = EidMlsEvolvement::OUT {
                message: mls_out.into(),
                welcome: Some(welcome),
            };
            Ok(evolvement)
        } else {
            Err(EidError::AddMemberError("No key package provided".into()))
        }
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
                .remove_members(&backend.mls_backend, &self.pubkey, &[mls_member.index])
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
        let (mls_out, _, _) = group
            .self_update(&backend.mls_backend, &self.pubkey)
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

    fn cross_sign_membership(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        self.update(backend)
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
            .export_group_info(&backend.mls_backend, &self.pubkey, false)
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
    fn generate_initial_id(id: Vec<u8>, backend: &Self::BackendProvider) -> Self::MemberProvider {
        let ciphersuite = backend.ciphersuite;
        let (cred_with_key, sig_pubkey) = create_store_credential(
            id,
            CredentialType::Basic,
            ciphersuite.signature_algorithm(),
            &backend.mls_backend,
        );

        let key_bundle = create_store_key_package(
            ciphersuite,
            cred_with_key.clone(),
            &backend.mls_backend,
            &sig_pubkey,
        );

        // TODO: we're basically throwing away the private parts (but they're stored in the key store) - do we want this?
        EidMlsMember {
            mls_member: None,
            key_package: Some(key_bundle.clone()),
            credential: cred_with_key,
        }
    }
}
