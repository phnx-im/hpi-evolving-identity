use openmls::prelude::{
    Ciphersuite, Credential, CredentialType, CredentialWithKey, CryptoConfig, KeyPackage,
    OpenMlsCryptoProvider, SignatureScheme,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;

use eid_traits::types::EidError;

pub(crate) fn create_store_credential(
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

pub(crate) fn create_store_key_package(
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
