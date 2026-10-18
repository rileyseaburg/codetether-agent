//! Compatibility exports for the shared owning Bedrock provider.
//!
//! The provider crate owns construction, authentication recovery, request
//! routing and the Provider implementation alongside its borrowed HTTP client.
pub use codetether_provider_bedrock::provider::*;
