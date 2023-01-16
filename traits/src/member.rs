use std::fmt::Debug;

pub trait Member: PartialEq + Debug {
    type IdentityProvider;

    fn new(id: Self::IdentityProvider) -> Self;

    fn get_identity(&self) -> Self::IdentityProvider;
}
