pub use eid_dummy::eid_dummy_client::EidDummyClient;
use eid_traits::client::EidClient;
pub use rstest::*;
pub use rstest_reuse::{self, *};

#[rstest(client,
case::EIDDummy(&EidDummyClient::default()),
)
]
pub fn eid_clients(client: &impl EidClient) {}
