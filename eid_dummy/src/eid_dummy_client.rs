use eid_traits::client::EidClient;
use eid_traits::state::EidState;
use eid_traits::types::EidError;

use crate::eid_dummy_backend::EidDummyBackend;
use crate::eid_dummy_evolvement::EidDummyEvolvement;
use crate::eid_dummy_member::{EidDummyMember, BOOLEAN};
use crate::eid_dummy_state::EidDummyState;
use crate::eid_dummy_transcript::EidDummyTranscript;

pub struct EidDummyClient {
    state: EidDummyState,
    id: Vec<u8>,
}

impl EidClient for EidDummyClient {
    type EvolvementProvider = EidDummyEvolvement;
    type MemberProvider = EidDummyMember;
    type TranscriptStateProvider = EidDummyState;
    type ExportedTranscriptStateProvider = EidDummyState;
    type BackendProvider = EidDummyBackend;
    type KeyProvider = ();

    // We're only requiring this for tests since we don't want to unnecessarily restrict transcript tue transcript type.
    #[cfg(feature = "test")]
    type TranscriptProvider = EidDummyTranscript;

    fn create_eid(
        initial_member: &Self::MemberProvider,
        _keypair: Self::KeyProvider,
        _backend: &Self::BackendProvider,
    ) -> Result<Self, EidError> {
        let members = vec![initial_member.clone()];
        let state = EidDummyState { members };
        Ok(EidDummyClient {
            state,
            id: initial_member.id.clone(),
        })
    }

    fn create_from_invitation(
        invitation: Self::EvolvementProvider,
        _keypair: Self::KeyProvider,
        _backend: &Self::BackendProvider,
    ) -> Result<Self, EidError>
    where
        Self: Sized,
    {
        if let EidDummyEvolvement::Add {
            members,
            invited_id: invited_pk,
        } = invitation
        {
            Ok(Self {
                state: EidDummyState { members },
                id: invited_pk,
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
            invited_id: member.id.clone(),
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
        let new_pk: Vec<u8> = (0..256).map(|_| rand::random::<u8>()).collect();

        let mut new_members = self.state.members.clone();
        // remove yourself from member list
        let myself = self.state.members.iter().find(|&x| x.id == self.id);
        let mut myself = myself.unwrap().clone();
        new_members.retain(|m| &myself != m);
        myself.cross_signed = BOOLEAN::TRUE;
        myself.pk = new_pk;
        new_members.push(myself);

        // create an evolvement with the new member
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
        self.state
            .get_members()
            .iter()
            .filter(|&m| m.cross_signed == BOOLEAN::TRUE)
            .map(|m| m.clone())
            .collect()
    }
    fn export_transcript_state(
        &self,
        _backend: &Self::BackendProvider,
    ) -> Result<EidDummyState, EidError> {
        Ok(self.state.clone())
    }

    #[cfg(feature = "test")]
    fn generate_initial_member(
        id: Vec<u8>,
        _backend: &Self::BackendProvider,
    ) -> (Self::MemberProvider, Self::KeyProvider) {
        (
            EidDummyMember {
                id,
                pk: (0..256).map(|_| rand::random::<u8>()).collect(),
                cross_signed: BOOLEAN::FALSE,
            },
            (),
        )
    }

    #[cfg(feature = "test")]
    fn generate_initial_client(id: Vec<u8>, backend: &Self::BackendProvider) -> Self {
        let (member, keypair) = EidDummyClient::generate_initial_member(id, backend);
        EidDummyClient::create_eid(&member, keypair, backend).expect("Could not create EID")
    }
}
