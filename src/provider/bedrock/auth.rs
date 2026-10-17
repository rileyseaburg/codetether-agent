//! Compatibility exports for Bedrock credentials and shared bearer state.
//!
//! Use [AwsCredentials] for environment/file lookup and [BedrockAuth] to
//! choose IAM signing or a token that can be refreshed in place.
pub use codetether_bedrock_auth::{AwsCredentials, BedrockAuth};
