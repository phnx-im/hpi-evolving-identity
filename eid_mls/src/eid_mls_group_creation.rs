use eid_traits::types::EidError;
use openmls::group::MlsGroupConfig;
use openmls::prelude::{GroupId, KeyPackage, MlsGroup, OpenMlsCryptoProvider, PURE_PLAINTEXT_WIRE_FORMAT_POLICY, SenderRatchetConfiguration};

use crate::eid_mls_backend::EidMlsBackend;
use crate::eid_mls_client::EidMlsClient;
use crate::state::client_state::EidMlsClientState;

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
