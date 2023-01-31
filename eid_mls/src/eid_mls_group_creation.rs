use openmls::group::MlsGroupConfig;
use openmls::prelude::{
    CredentialWithKey, MlsGroup, SenderRatchetConfiguration, SignaturePublicKey,
    PURE_PLAINTEXT_WIRE_FORMAT_POLICY,
};
use openmls_basic_credential::SignatureKeyPair;

use eid_traits::client::EidClient;
use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_client::EidMlsClient;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_member::EidMlsMember;
use crate::state::client_state::EidMlsClientState;

impl EidMlsClient {
    pub(crate) fn gen_group_config() -> MlsGroupConfig {
        MlsGroupConfig::builder()
            .sender_ratchet_configuration(SenderRatchetConfiguration::new(10, 2000))
            .use_ratchet_tree_extension(true)
            .wire_format_policy(PURE_PLAINTEXT_WIRE_FORMAT_POLICY)
            .build()
    }

    pub(crate) fn create_mls_eid(
        backend: &EidMlsBackend,
        signature_key: SignaturePublicKey,
        credential: CredentialWithKey,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let mls_group_config = EidMlsClient::gen_group_config();

        let group = MlsGroup::new(
            &backend.mls_backend,
            &keypair,
            &mls_group_config,
            credential,
        )
        .expect("Could not create MlsGroup");

        Ok(Self {
            state: EidMlsClientState { group },
            pubkey: signature_key.clone(),
        })
    }
}
