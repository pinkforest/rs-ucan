// use anyhow::Result;
// use async_trait::async_trait;

/// This trait must be implemented by a struct that encapsulates cryptographic
/// keypair data. The trait represent the minimum required API capability for
/// producing a signed UCAN from a cryptographic keypair, and verifying such
/// signatures.
#[cfg(feature = "web")]
mod internal {
    use anyhow::Result;
    use async_trait::async_trait;

    #[async_trait(?Send)]
    pub trait KeyMaterial {
        fn get_jwt_algorithm_name(&self) -> String;
        fn get_did(&self) -> String;

        /// Sign some data with this key
        async fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;

        /// Verify the alleged signature of some data against this key
        async fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()>;
    }
}
#[cfg(not(feature = "web"))]
mod internal {
    use anyhow::Result;
    use async_trait::async_trait;

    #[async_trait]
    pub trait KeyMaterial: Sync + Send {
        fn get_jwt_algorithm_name(&self) -> String;
        fn get_did(&self) -> String;

        /// Sign some data with this key
        async fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;

        /// Verify the alleged signature of some data against this key
        async fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()>;
    }
}

pub use internal::*;
