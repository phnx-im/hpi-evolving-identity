use openmls::prelude::{
    Ciphersuite, Credential, CredentialType, CredentialWithKey, CryptoConfig, Extensions,
    KeyPackage, KeyPackageBuilder, OpenMlsCryptoProvider, ProtocolVersion, SignaturePublicKey,
    SignatureScheme,
};

pub(crate) fn create_store_credential(
    identity: Vec<u8>,
    credential_type: CredentialType,
    signature_algorithm: SignatureScheme,
    backend: &impl OpenMlsCryptoProvider,
) -> (CredentialWithKey, SignaturePublicKey) {
    let credential = Credential::new(identity, credential_type).unwrap();

    let signature_keys = SignatureKeyPair::new(signature_algorithm).unwrap();
    signature_keys.store(backend.key_store()).unwrap();

    (
        CredentialWithKey {
            credential,
            signature_key: signature_keys.to_public_vec().into(),
        },
        signature_keys,
    )
    /*
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

    return credential_bundle;*/
}

pub(crate) fn create_store_key_package(
    ciphersuite: Ciphersuite,
    credential_with_key: CredentialWithKey,
    backend: &impl OpenMlsCryptoProvider,
    signer: &impl Signer,
) -> KeyPackage {
    let kp = KeyPackage::builder()
        .key_package_extensions(extensions)
        .build(
            CryptoConfig::with_default_version(ciphersuite),
            backend,
            signer,
            credential_with_key,
        )
        .expect("Could not create KeyPackage");

    /*
    backend
        .key_store()
        .store(
            kp.hash_ref(backend.crypto())
                .expect("Could not hash KeyPackage")
                .as_slice(),
            &kp,
        )
        .expect("Storing KeyPackage failed");
     */

    return kp;
}
