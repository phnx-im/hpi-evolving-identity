use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::{EidError, Member};

use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_keystore::EidDummyKeystore;
use crate::eid_dummy_state::EidDummyState;

pub struct EidDummyClient<'a> {
    state: EidDummyState,
    key_store: &'a EidDummyKeystore,
    pk: Vec<u8>,
    pending_pk_update: Option<Vec<u8>>,
}

impl<'a> EidClient<'a> for EidDummyClient<'a> {
    type StateProvider = EidDummyState;
    type KeyStoreProvider = EidDummyKeystore;
    type EvolvementProvider = EidDummyEvolvement;

    fn state(&self) -> &EidDummyState {
        &self.state
    }

    fn key_store(&self) -> &EidDummyKeystore {
        self.key_store
    }

    fn pk(&self) -> &[u8] {
        &self.pk
    }

    fn create_eid(key_store: &'a EidDummyKeystore) -> Result<Self, EidError> {
        let pk = "public key".as_bytes().to_vec();
        let members = vec![Member::new(pk.clone())];
        let state = EidDummyState { members };
        Ok(EidDummyClient {
            state,
            key_store,
            pk,
            pending_pk_update: None,
        })
    }

    fn evolve(
        &mut self,
        evolvement: &EidDummyEvolvement,
        _backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        // in case of update, change your own pk
        if let EidDummyEvolvement::Update { members: _ } = &evolvement {
            self.pk = self.pending_pk_update.clone().unwrap();
            self.pending_pk_update = None;
        }

        self.state.apply(evolvement)
    }

    fn add(
        &self,
        member: &Member,
        _backend: &EidDummyBackend,
    ) -> Result<EidDummyEvolvement, EidError> {
        if self.state.members.contains(member) {
            return Err(EidError::AddMemberError(String::from(
                "Member already in EID",
            )));
        }
        let mut new_state = self.state.clone();
        new_state.members.push(member.clone());
        let evolvement = EidDummyEvolvement::Add {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn remove(
        &self,
        member: &Member,
        _backend: &EidDummyBackend,
    ) -> Result<EidDummyEvolvement, EidError> {
        if !self.state.members.contains(member) {
            return Err(EidError::InvalidMemberError(String::from(
                "Member not in EID",
            )));
        }

        let mut new_state = self.state.clone();

        if let Some(pos) = new_state.members.iter().position(|x| x.pk() == member.pk()) {
            new_state.members.swap_remove(pos);
        }

        let evolvement = EidDummyEvolvement::Remove {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn update(&mut self) -> Result<EidDummyEvolvement, EidError> {
        let mut new_members = self.state.members.clone();
        // remove yourself from member list
        new_members.retain(|m| *self.pk() != m.pk());

        // create a member with your new pk
        let mut new_pk = self.pk().to_vec();
        new_pk[0] += 1;
        let member = Member::new(new_pk.clone());

        // remember the new pk for later
        self.pending_pk_update = Some(new_pk);

        // create an evolvement with the new member
        new_members.push(member);
        let evolvement = EidDummyEvolvement::Update {
            members: new_members,
        };
        Ok(evolvement)
    }
}
