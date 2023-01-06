pub trait Member: PartialEq {
    type CredentialProvider;

    fn new(cred: Self::CredentialProvider) -> Self;
    fn get_pk(&self) -> Vec<u8>;
}
