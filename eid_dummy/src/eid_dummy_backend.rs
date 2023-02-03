use eid_traits::backend::EidBackend;

use crate::eid_dummy_client::EidDummyClient;

#[derive(Default)]
pub struct EidDummyBackend {}

impl EidBackend for EidDummyBackend {
    #[cfg(feature = "test")]
    type ClientProvider = EidDummyClient;
}
