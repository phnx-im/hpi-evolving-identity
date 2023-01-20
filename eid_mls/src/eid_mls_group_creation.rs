use openmls::group::MlsGroupConfig;
use openmls::prelude::{MlsGroup, SenderRatchetConfiguration, PURE_PLAINTEXT_WIRE_FORMAT_POLICY};

use eid_traits::types::EidError;

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_client::EidMlsClient;
use crate::eid_mls_member::EidMlsMember;
use crate::state::client_state::EidMlsClientState;

impl EidMlsClient {
    pub(crate) fn create_mls_eid(
        backend: &EidMlsBackend,
        member: &EidMlsMember,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        let mls_group_config = MlsGroupConfig::builder()
            .sender_ratchet_configuration(SenderRatchetConfiguration::new(10, 2000))
            .use_ratchet_tree_extension(true)
            .wire_format_policy(PURE_PLAINTEXT_WIRE_FORMAT_POLICY)
            .build();

        let signature_key = member.key_package.leaf_node().signature_key().clone();

        let group = MlsGroup::new(&backend.mls_backend, &mls_group_config, &signature_key)
            .expect("Could not create MlsGroup");

        let members = vec![member.clone()];

        Ok(Self {
            state: EidMlsClientState { group, members },
        })
    }
}
