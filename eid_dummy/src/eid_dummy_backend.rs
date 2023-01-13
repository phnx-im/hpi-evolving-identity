use eid_traits::backend::EidBackend;

#[derive(Default)]
pub struct EidDummyBackend {}

impl EidBackend for EidDummyBackend {}