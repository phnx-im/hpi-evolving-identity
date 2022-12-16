use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};
use openmls::prelude::Ciphersuite;
use openmls_rust_crypto::OpenMlsRustCrypto;

use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::state::client_state::EidMlsClientState;

pub struct EidMlsClient {
    pub(crate) state: EidMlsClientState,
    pub(crate) backend: &'static OpenMlsRustCrypto,
}

impl<'a> EidClient<'a> for EidMlsClient {
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidMlsEvolvement;
    type StateProvider = EidMlsClientState;
    type Member = KeyPackage;

    fn state(&mut self) -> &mut Self::StateProvider {
        todo!()
    }

    fn key_store(&self) -> &Self::KeyStoreProvider {
        todo!()
    }

    fn pk(&self) -> &[u8] {
        todo!()
    }

    fn create_eid(keystore: &'a Self::KeyStoreProvider) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        // Define cipher suite ...
        let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;
        // ... and the crypto backend to use.
        let backend = &OpenMlsRustCrypto::default();

        Self::create_mls_eid(keystore, backend, ciphersuite)
    }

    fn add(&mut self, member: &Self::Member) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized,
    {
        let group = &mut self.state.group;
        let (mls_out, welcome) = group.add_members(self.backend, &[member.clone()]);
        let mls_in = mls_out.into();
        let proc_msg = group
            .process_unverified_message(mls_in, None, backend)
            .expect("Can't process message");
        return if let ProcessedMessage::StagedCommitMessage(staged_commit) = proc_msg {
            Ok(EidMlsEvolvement {
                commit: *staged_commit,
                message: mls_in,
            })
        } else {
            Err(e)
        };
    }

    fn remove(&self, member: &Self::Member) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(&mut self) -> Result<Self::EvolvementProvider, EidError> {
        todo!()
    }

    fn evolve(&mut self, evolvement: &Self::EvolvementProvider) -> Result<(), EidError> {
        todo!()
    }
}
