use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};

use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
struct EidDummyClient {
    state: EidDummyState,
    key_store: EidDummyKeystore,
}

impl EidClient for EidDummyClient {
    type StateProvider = EidDummyState;
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidDummyEvolvement;

    fn state(&self) -> &EidDummyState {
        &self.state
    }

    fn key_store(&self) -> &EidDummyKeystore { &self.key_store }

    fn create_eid(key_store: EidDummyKeystore) -> Result<Self, EidError> {
        let members = vec![Member::default()];
        let state = EidDummyState {
            members
        };
        Ok(EidDummyClient {
            state,
            key_store,
        })
    }
    fn add(&mut self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        let mut new_state = self.state.clone();
        new_state.members.push(member);
        let evolvement = EidDummyEvolvement {members: new_state.members};
        Ok(evolvement)
    }
    fn remove(&mut self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        let mut new_state = self.state.clone();

        if let Some(pos) = new_state.members.iter().position(|x| *x.pk() == *member.pk()) {
            new_state.members.swap_remove(pos);
        }

        let evolvement = EidDummyEvolvement {members: new_state.members};
        Ok(evolvement)
    }
    fn update(&mut self) -> Result<EidDummyEvolvement, EidError> {
        let mut new_state = self.state.clone();
        new_state.members.retain(|&m| self.pk() == m.pk());

        let evolvement = EidDummyEvolvement {members: new_state.members};
        Ok(evolvement)
    }
}