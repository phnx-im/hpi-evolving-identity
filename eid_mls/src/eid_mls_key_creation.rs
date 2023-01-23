use openmls::key_packages::KeyPackageBuilder;
use openmls::prelude::KeyPackage;
use openmls::prelude::{
    Ciphersuite, CredentialBundle, CredentialType, CryptoConfig, OpenMlsCryptoProvider,
    OpenMlsKeyStore, ProtocolVersion, SignatureScheme, TlsSerializeTrait,
};

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
) -> KeyPackage {
    let kp = KeyPackageBuilder::new()
        .build(
            CryptoConfig {
                ciphersuite,
                version: ProtocolVersion::default(),
            },
            backend,
            credential_bundle,
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
