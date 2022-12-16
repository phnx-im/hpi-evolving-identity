use eid_traits::state::EidState;
use eid_traits::types::EidError;
use eid_traits::types::Member;
use openmls::framing::{ProcessedMessage, UnverifiedMessage};
use openmls::group::MlsGroup;
use openmls::prelude::{OpenMlsCrypto, OpenMlsCryptoProvider};
use openmls_rust_crypto::OpenMlsRustCrypto;

use crate::eid_mls_client::EidMlsClient;
use crate::eid_mls_evolvement::EidMlsEvolvement;

pub(crate) trait EidMlsState: EidState<T> + Clone + PartialEq {
    fn verify_client(&self, client: &Member) -> Result<bool, EidError> {
        let members = self.get_members()?;
        Ok(members.contains(client))
    }

    fn apply_log(&mut self, log: &Vec<EidMlsEvolvement>) -> Result<(), EidError>
    where
        Self: Sized,
    {
        for evolvement in log.iter() {
            self.apply(evolvement)?;
        }
        Ok(())
    }
}
