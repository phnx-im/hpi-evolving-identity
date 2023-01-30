use openmls::group::MlsGroupConfig;
use openmls::prelude::{
    MlsGroup, SenderRatchetConfiguration, SignaturePublicKey, PURE_PLAINTEXT_WIRE_FORMAT_POLICY,
};

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
        member: &EidMlsMember,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let mls_group_config = EidMlsClient::gen_group_config();

        let key_package = member
            .key_package
            .clone()
            .ok_or(EidError::InvalidMemberError(
                "No key package provided in member".into(),
            ))?;

        let signature_key = key_package.leaf_node().signature_key();

        let group = MlsGroup::new(&backend.mls_backend, &mls_group_config, &signature_key)
            .expect("Could not create MlsGroup");

        Ok(Self {
            state: EidMlsClientState { group },
        })
    }
}
