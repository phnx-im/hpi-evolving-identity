use openmls::prelude::{Ciphersuite, CredentialBundle, CredentialType, Extension, KeyPackageBundle, OpenMlsCryptoProvider, OpenMlsKeyStore, SignatureScheme, TlsSerializeTrait};

pub(crate) fn create_store_credential(
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

pub(crate) fn create_store_key_package(
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