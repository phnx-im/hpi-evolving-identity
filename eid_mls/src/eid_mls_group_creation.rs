use eid_traits::types::EidError;
use openmls::prelude::{Ciphersuite, CredentialBundle, CredentialType, Extension, GroupId, KeyPackage, KeyPackageBundle, LifetimeExtension, MlsGroup, MlsGroupConfig, OpenMlsCryptoProvider, OpenMlsKeyStore, PURE_PLAINTEXT_WIRE_FORMAT_POLICY, SenderRatchetConfiguration, SignatureScheme, TlsSerializeTrait};

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_client::EidMlsClient;
use crate::state::client_state::EidMlsClientState;

fn create_store_credential(
    identifier: String,
    backend: &impl OpenMlsCryptoProvider,
    signature_scheme: SignatureScheme,
) -> CredentialBundle {
    let credential_bundle = CredentialBundle::new(
        identifier.into(),
        CredentialType::Basic,
        signature_scheme,
        backend,
    )
    .expect("Could not create CredentialBundle");

    let credential = credential_bundle.credential().clone();
    backend
        .key_store()
        .store(
            &credential
                .signature_key()
                .tls_serialize_detached()
                .expect("Error serialising signature key"),
            &credential_bundle,
        )
        .expect("Storing credential failed");

    return credential_bundle;
}

fn create_store_key_package(
    ciphersuite: Ciphersuite,
    credential_bundle: &CredentialBundle,
    backend: &impl OpenMlsCryptoProvider,
    extensions: Vec<Extension>,
) -> KeyPackageBundle {
    let key_package_bundle = KeyPackageBundle::new(
        &[ciphersuite],
        credential_bundle,
        backend,
        extensions.clone(),
    )
    .expect("Could not create KeyPackage");

    let key_package = key_package_bundle.key_package().clone();
    backend
        .key_store()
        .store(
            key_package
                .hash_ref(backend.crypto())
                .expect("Could not hash KeyPackage")
                .as_slice(),
            &key_package_bundle,
        )
        .expect("Storing KeyPackage failed");

    return key_package_bundle;
}

impl EidMlsClient {
    pub(crate) fn create_mls_eid(
        backend: &EidMlsBackend,
        key_package: &KeyPackage,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let mls_group_config = MlsGroupConfig::builder()
            .sender_ratchet_configuration(SenderRatchetConfiguration::new(10, 2000))
            .use_ratchet_tree_extension(true)
            .wire_format_policy(PURE_PLAINTEXT_WIRE_FORMAT_POLICY)
            .build();

        let mut mls_group = MlsGroup::new(
            &backend.mls_backend,
            &mls_group_config,
            GroupId::from_slice(b"group01"), // TODO: set some actual identifier
            key_package
                .hash_ref(backend.mls_backend.crypto())
                .expect("Could not hash KeyPackage")
                .as_slice(),
        )
        .expect("Could not create MlsGroup");

        Ok(Self {
            state: EidMlsClientState { group: mls_group },
        })
    }
}
