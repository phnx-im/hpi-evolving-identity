use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::EidDummyMember;
use crate::eid_dummy_state::EidDummyState;

pub struct EidDummyClient {
    state: EidDummyState,
    pk: Vec<u8>,
    pending_pk_update: Option<Vec<u8>>,
}

impl EidClient for EidDummyClient {
    type ClientStateProvider = EidDummyState;
    type TranscriptStateProvider = EidDummyState;
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type BackendProvider = EidDummyBackend;

    fn generate_credential(
        _backend: &Self::BackendProvider,
    ) -> <Self::MemberProvider as Member>::CredentialProvider {
        (0..256).map(|_| rand::random::<u8>()).collect()
    }
    fn export_transcript_state(&self) -> EidDummyState {
        self.state.clone()
    }

    fn pk(&self) -> &[u8] {
        &self.pk
    }

    fn create_eid(
        cred: <Self::MemberProvider as Member>::CredentialProvider,
        _backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let members = vec![EidDummyMember::new(cred.clone())];
        let state = EidDummyState { members };
        Ok(EidDummyClient {
            state,
            pk: cred,
            pending_pk_update: None,
        })
    }

    fn evolve(
        &mut self,
        evolvement: &EidDummyEvolvement,
        backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        // in case of update, change your own pk
        if let EidDummyEvolvement::Update { members: _ } = &evolvement {
            self.pk = self.pending_pk_update.clone().unwrap();
            self.pending_pk_update = None;
        }

        self.state.apply(evolvement, backend)
    }

    fn add(
        &mut self,
        member: &EidDummyMember,
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
        &mut self,
        member: &EidDummyMember,
        _backend: &EidDummyBackend,
    ) -> Result<EidDummyEvolvement, EidError> {
        if !self.state.members.contains(member) {
            return Err(EidError::InvalidMemberError(String::from(
                "Member not in EID",
            )));
        }

        let mut new_state = self.state.clone();

        if let Some(pos) = new_state.members.iter().position(|x| x == member) {
            new_state.members.swap_remove(pos);
        }

        let evolvement = EidDummyEvolvement::Remove {
            members: new_state.members,
        };
        Ok(evolvement)
    }
    fn update(&mut self, _backend: &EidDummyBackend) -> Result<EidDummyEvolvement, EidError> {
        let mut new_members = self.state.members.clone();
        // remove yourself from member list
        let myself = &EidDummyMember::new(self.pk().into());
        new_members.retain(|m| myself != m);

        // create a member with your new pk
        let mut new_pk = self.pk().to_vec();
        new_pk[0] += 1;
        let member = EidDummyMember::new(new_pk.clone());

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
