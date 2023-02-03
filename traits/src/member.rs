use std::fmt::Debug;

pub trait Member: PartialEq + Debug + Clone {
    type CredentialProvider;

    fn new(id: Self::CredentialProvider) -> Self;

    #[cfg(feature = "test")]
    fn get_credential(&self) -> Self::CredentialProvider;
}
