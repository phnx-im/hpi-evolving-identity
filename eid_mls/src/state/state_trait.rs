use openmls::prelude::ProcessedMessage;

use eid_traits::state::EidState;
use eid_traits::types::EidError;

pub trait EidMlsState: EidState + Clone + PartialEq {
    fn verify_member(&self, member: &Self::MemberProvider) -> Result<bool, EidError> {
        let members = self.get_members()?;
        Ok(members.contains(client))
    }

    fn apply_log(
        &mut self,
        log: &Vec<Self::EvolvementProvider>,
        backend: &Self::BackendProvider,
    ) -> Result<(), EidError>
    where
        Self: Sized,
    {
        for evolvement in log.iter() {
            self.apply(evolvement)?;
        }
        Ok(())
    }

    fn apply_processed_message(&mut self, message: ProcessedMessage) -> Result<(), EidError> {
        todo!()
    }
}
