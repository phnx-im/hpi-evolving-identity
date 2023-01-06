pub trait Member: PartialEq {
    type CredentialProvider;

    fn new(cred: Self::CredentialProvider) -> Self;

    fn get_credential(&self) -> Self::CredentialProvider;
}
