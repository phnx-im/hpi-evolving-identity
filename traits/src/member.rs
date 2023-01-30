use std::fmt::Debug;

pub trait Member: PartialEq + Debug + Clone {
    type IdentityProvider;

    fn new(id: Self::IdentityProvider) -> Self;

    #[cfg(feature = "test")]
    fn get_identity(&self) -> Self::IdentityProvider;
}
