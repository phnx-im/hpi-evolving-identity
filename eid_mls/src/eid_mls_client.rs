use openmls::prelude::{
    Credential, CredentialType, CredentialWithKey, CryptoConfig, KeyPackage, MlsGroup,
};
use openmls::prelude::{
    MlsGroupConfig, MlsMessageInBody, SenderRatchetConfiguration, PURE_PLAINTEXT_WIRE_FORMAT_POLICY,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;
use openmls_traits::types::{Ciphersuite, SignatureScheme};
use openmls_traits::OpenMlsCryptoProvider;

use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::eid_mls_transcript::EidMlsTranscript;
use crate::state::client_state::EidMlsClientState;
use crate::state::transcript_state::{EidMlsExportedTranscriptState, EidMlsTranscriptState};

/// # EID MLS Client
/// Implementation of [EidClient] using [openmls]. Uses [EidMlsClientState] as its state.
pub struct EidMlsClient {
    pub(crate) state: EidMlsClientState,
    pub(crate) key_pair: SignatureKeyPair,
}

impl EidClient for EidMlsClient {
    type EvolvementProvider = EidMlsEvolvement;
    type MemberProvider = EidMlsMember;
    type TranscriptStateProvider = EidMlsTranscriptState;
    type ExportedTranscriptStateProvider = EidMlsExportedTranscriptState;
    type BackendProvider = EidMlsBackend;
    type KeyProvider = SignatureKeyPair;

    #[cfg(feature = "test")]
    type TranscriptProvider = EidMlsTranscript;

    fn create_eid(
        initial_member: &Self::MemberProvider,
        key_pair: Self::KeyProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let mls_group_config = Self::gen_group_config();

        let group = MlsGroup::new(
            &backend.mls_backend,
            &key_pair,
            &mls_group_config,
            initial_member.credential.clone(),
        )
        .expect("Could not create MlsGroup");

        Ok(Self {
            state: EidMlsClientState { group },
            key_pair,
        })
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
                .map_err(|err| EidError::CreateClientError(err.to_string()))?;
                return Ok(Self {
                    state: EidMlsClientState { group: mls_group },
                    key_pair: signature_keypair,
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
                .add_members(&backend.mls_backend, &self.key_pair, &[key_package])
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
                .remove_members(&backend.mls_backend, &self.key_pair, &[mls_member.index])
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
            .self_update(&backend.mls_backend, &self.key_pair)
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
            .export_group_info(&backend.mls_backend, &self.key_pair, false)
            .map_err(|_| EidError::ExportTranscriptStateError)?;
        let nodes = self.state.group.export_ratchet_tree().into();

        Ok(EidMlsExportedTranscriptState::OUT {
            group_info: mls_out,
            nodes,
        })
    }

    #[cfg(feature = "test")]
    fn generate_member(
        id: Vec<u8>,
        backend: &Self::BackendProvider,
    ) -> (Self::MemberProvider, Self::KeyProvider) {
        let ciphersuite = backend.ciphersuite;
        let (cred_with_key, keypair) = create_store_credential(
            id,
            CredentialType::Basic,
            ciphersuite.signature_algorithm(),
            &backend.mls_backend,
        )
        .expect("Failed to create credential");

        let key_package = Self::create_store_key_package(
            ciphersuite,
            cred_with_key.clone(),
            &backend.mls_backend,
            &keypair,
        )
        .expect("Failed to create key package");

        (
            EidMlsMember {
                mls_member: None,
                key_package: Some(key_package.clone()),
                credential: cred_with_key,
            },
            keypair,
        )
    }

    #[cfg(feature = "test")]
    fn generate_initial_client(id: Vec<u8>, backend: &Self::BackendProvider) -> Self {
        let (member, keypair) = Self::generate_member(id, backend);
        Self::create_eid(&member, keypair, backend).expect("Could not create EID")
    }
}

impl EidMlsClient {
    fn gen_group_config() -> MlsGroupConfig {
        MlsGroupConfig::builder()
            .sender_ratchet_configuration(SenderRatchetConfiguration::new(10, 2000))
            .use_ratchet_tree_extension(true)
            .wire_format_policy(PURE_PLAINTEXT_WIRE_FORMAT_POLICY)
            .build()
    }

    fn create_store_credential(
        identity: Vec<u8>,
        credential_type: CredentialType,
        signature_algorithm: SignatureScheme,
        backend: &impl OpenMlsCryptoProvider,
    ) -> Result<(CredentialWithKey, SignatureKeyPair), EidError> {
        let credential = Credential::new(identity, credential_type)
            .map_err(|e| EidError::CreateCredentialError(e.to_string()))?;
        let signature_keys = SignatureKeyPair::new(signature_algorithm)
            .map_err(|e| EidError::CreateCredentialError(e.to_string()))?;
        signature_keys
            .store(backend.key_store())
            .map_err(|e| EidError::CreateCredentialError(e.to_string()))?;

        Ok((
            CredentialWithKey {
                credential,
                signature_key: signature_keys.to_public_vec().into(),
            },
            signature_keys,
        ))
    }

    fn create_store_key_package(
        ciphersuite: Ciphersuite,
        credential_with_key: CredentialWithKey,
        backend: &impl OpenMlsCryptoProvider,
        signer: &impl Signer,
    ) -> Result<KeyPackage, EidError> {
        let kp = KeyPackage::builder()
            //.key_package_extensions(extensions)
            .build(
                CryptoConfig::with_default_version(ciphersuite),
                backend,
                signer,
                credential_with_key,
            )
            .map_err(|e| EidError::CreateCredentialError(e.to_string()))?;

        return Ok(kp);
    }
}
