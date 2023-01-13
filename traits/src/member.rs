use std::fmt::Debug;

pub trait Member: PartialEq + Debug {
    type PubkeyProvider;

    fn new(cred: Self::PubkeyProvider) -> Self;

    fn get_pk(&self) -> Self::PubkeyProvider;
}
