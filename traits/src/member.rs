use std::fmt::Debug;

/// An EID Member representing a member in the EID. A member can be added or removed from the EID.
pub trait Member: PartialEq + Debug + Clone {
    /// Type that can be used to identify a member
    type CredentialProvider;

    /// Create a [Member]
    ///
    /// # Arguments
    ///
    /// * `id`: The [Self::CredentialProvider]
    ///
    /// returns: Self
    fn new(id: Self::CredentialProvider) -> Self;

    /// Get the signature public key of a member
    #[cfg(feature = "test")]
    fn get_pk(&self) -> Vec<u8>;
}
