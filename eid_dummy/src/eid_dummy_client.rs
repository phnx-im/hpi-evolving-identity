use eid_traits::client::EidClient;
use eid_traits::key_store::{EidKeyStore, ToKeyStoreValue};
use eid_traits::types::{EidError, Member};

use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_keypair::KeyPair;
use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
pub struct EidDummyClient {
    state: EidDummyState,
    key_store: EidDummyKeystore,
}

impl EidClient for EidDummyClient {
    type StateProvider = EidDummyState;
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidDummyEvolvement;

    fn state(&mut self) -> &mut EidDummyState {
        &mut self.state
    }

    fn key_store(&self) -> &EidDummyKeystore {
        &self.key_store
    }

    fn pk(&self) -> Vec<u8> {
        let keypair: KeyPair = self.key_store().read("some key".as_bytes()).unwrap();
        keypair.to_key_store_value().unwrap()
    }

    fn create_eid(key_store: EidDummyKeystore) -> Result<Self, EidError> {
        let members = vec![Member::default()];
        let state = EidDummyState { members };
        Ok(EidDummyClient { state, key_store })
    }
    fn add(&self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        let mut new_state = self.state.clone();
        new_state.members.push(member);
        let evolvement = EidDummyEvolvement {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn remove(&self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        let mut new_state = self.state.clone();

        if let Some(pos) = new_state.members.iter().position(|x| x.pk() == member.pk()) {
            new_state.members.swap_remove(pos);
        }

        let evolvement = EidDummyEvolvement {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn update(&self) -> Result<EidDummyEvolvement, EidError> {
        let mut new_members = self.state.members.clone();
        new_members.retain(|m| self.pk() != m.pk());

        let mut new_pk = self.pk().clone();
        new_pk[0] = new_pk[0] + 1;
        let member = Member::new(new_pk);

        new_members.push(member);
        let evolvement = EidDummyEvolvement {
            members: new_members,
        };
        Ok(evolvement)
    }
}
