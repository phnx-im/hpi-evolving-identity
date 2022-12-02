use eid_traits::client::EidClient;
use eid_traits::types::{EidError, Member};

use crate::eid_dummy_evolvement::EidDummyEvolvement;

use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_dummy_state::EidDummyState;

#[derive(Default)]
pub struct EidDummyClient {
    state: EidDummyState,
    key_store: EidDummyKeystore,
    pk: Vec<u8>,
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

    fn pk(&self) -> &Vec<u8> {
        &self.pk
    }

    fn create_eid(key_store: EidDummyKeystore) -> Result<Self, EidError> {
        let pk = "public key".as_bytes().to_vec();
        let members = vec![Member::new(pk.clone())];
        let state = EidDummyState { members };
        Ok(EidDummyClient {
            state,
            key_store,
            pk: pk,
        })
    }
    fn add(&self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        if self.state.members.contains(&member) {
            return Err(EidError::AddMemberError(String::from(
                "Member already in EID",
            )));
        }
        let mut new_state = self.state.clone();
        new_state.members.push(member);
        let evolvement = EidDummyEvolvement {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn remove(&self, member: Member) -> Result<EidDummyEvolvement, EidError> {
        if !self.state.members.contains(&member) {
            return Err(EidError::InvalidMemberError(String::from(
                "Member not in EID",
            )));
        }

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
        // remove yourself from member list
        new_members.retain(|m| *self.pk() != m.pk());

        // create a member with your new pk
        let mut new_pk = self.pk().clone();
        new_pk[0] = new_pk[0] + 1;
        let member = Member::new(new_pk);

        // create an evolvement with the new member
        new_members.push(member);
        let evolvement = EidDummyEvolvement {
            members: new_members,
        };
        Ok(evolvement)
    }
}
