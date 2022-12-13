use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};
use openmls::group::MlsGroup;
use openmls::prelude::StagedCommit;

use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_mls_evolvement::EidMlsEvolvement;
use crate::eid_mls_state::EidMlsState;

#[derive(Default)]
struct EidMlsClient {
    state: EidMlsState,
}

impl<'a> EidClient<'a> for EidMlsClient {
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidMlsEvolvement;
    type StateProvider = EidMlsState;

    fn state(&mut self) -> &mut Self::StateProvider {
        todo!()
    }

    fn key_store(&self) -> &Self::KeyStoreProvider {
        todo!()
    }

    fn pk(&self) -> &Vec<u8> {
        todo!()
    }

    fn create_eid(keystore: &Self::KeyStoreProvider) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn add(&self, member: &Member) -> Result<Self::EvolvementProvider, EidError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn remove(&self, member: &Member) -> Result<Self::EvolvementProvider, EidError>
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
