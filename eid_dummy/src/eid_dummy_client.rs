use serde_json::json_internal_vec;

use eid_traits::client::EidClient;
use eid_traits::member::Member;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::{EidDummyMember, BOOLEAN};
use crate::eid_dummy_state::EidDummyState;
use crate::eid_dummy_transcript::EidDummyTranscript;

pub struct EidDummyClient {
    state: EidDummyState,
    pk: Vec<u8>,
    pending_pk_update: Option<Vec<u8>>,
}

impl EidClient for EidDummyClient {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type TranscriptStateProvider = EidDummyState;
    type ExportedTranscriptStateProvider = EidDummyState;
    type BackendProvider = EidDummyBackend;

    // We're only requiring this for tests since we don't want to unnecessarily restrict transcript tue transcript type.
    #[cfg(feature = "test")]
    type TranscriptProvider = EidDummyTranscript;

    fn create_eid(
        identity: &Self::MemberProvider,
        _backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let members = vec![identity.clone()];
        let state = EidDummyState { members };
        Ok(EidDummyClient {
            state,
            pk: identity.get_identity(),
            pending_pk_update: None,
        })
    }

    fn create_from_invitation(
        invitation: Self::EvolvementProvider,
        backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        if let EidDummyEvolvement::Add {
            members,
            invited_pk,
        } = invitation
        {
            Ok(Self {
                state: EidDummyState { members },
                pk: invited_pk,
                pending_pk_update: None,
            })
        } else {
            Err(EidError::InvalidInvitationError)
        }
    }

    fn add(
        &mut self,
        member: &EidDummyMember,
        _backend: &EidDummyBackend,
    ) -> Result<EidDummyEvolvement, EidError> {
        if self.state.members.contains(&member) {
            return Err(EidError::AddMemberError(String::from(
                "Member already in EID",
            )));
        }
        let mut new_state = self.state.clone();
        new_state.members.push(member.clone());
        let evolvement = EidDummyEvolvement::Add {
            members: new_state.members,
            invited_pk: member.pk.clone(),
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
        // create a member with your new pk
        let mut new_pk = self.pk.to_vec();
        new_pk[0] += 1;

        // remember the new pk for later
        self.pending_pk_update = Some(new_pk.clone());

        let mut new_members = self.state.members.clone();
        // remove yourself from member list
        let myself = &EidDummyMember::new(self.pk.clone());
        new_members.retain(|m| myself != m);

        let mut member = EidDummyMember::new(new_pk.clone());
        member.cross_signed = BOOLEAN::TRUE;
        // create an evolvement with the new member
        new_members.push(member);
        let evolvement = EidDummyEvolvement::Update {
            members: new_members,
        };
        Ok(evolvement)
    }

    fn evolve(
        &mut self,
        evolvement: EidDummyEvolvement,
        backend: &EidDummyBackend,
    ) -> Result<(), EidError> {
        // in case of update, change your own pk
        if let EidDummyEvolvement::Update { members: _ } = &evolvement {
            self.pk = self.pending_pk_update.clone().unwrap();
            self.pending_pk_update = None;
        }

        self.state.apply(evolvement, backend)
    }

    fn cross_sign_membership(
        &mut self,
        backend: &Self::BackendProvider,
    ) -> Result<Self::EvolvementProvider, EidError> {
        // update yourself in the member list and set cross signed to true
        self.update(backend)
    }

    fn get_members(&self) -> Vec<Self::MemberProvider> {
        self.state.get_members()
    }
    fn export_transcript_state(
        &self,
        _backend: &Self::BackendProvider,
    ) -> Result<EidDummyState, EidError> {
        Ok(self.state.clone())
    }

    #[cfg(feature = "test")]
    fn generate_initial_id(_id: String, _backend: &Self::BackendProvider) -> Self::MemberProvider {
        EidDummyMember {
            pk: (0..256).map(|_| rand::random::<u8>()).collect(),
            cross_signed: BOOLEAN::FALSE,
        }
    }
}
