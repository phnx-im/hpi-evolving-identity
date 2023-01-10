pub trait Member: PartialEq {
    type PubkeyProvider;

    fn new(cred: Self::PubkeyProvider) -> Self;

    fn get_pk(&self) -> Self::PubkeyProvider;
}
