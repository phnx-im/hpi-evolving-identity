pub use rstest::*;
pub use rstest_reuse::{self, *};
use eid_traits::client::EidClient;

#[rstest(client,
case::rust_crypto(&EidMlsClient::default()),
)
]
pub fn eid_clients(client: &impl EidClient) {}