//! IAM or shared live bearer-token authentication.
use crate::*;

/// Authentication mode for the Bedrock provider.
///
/// Bedrock supports two auth modes:
/// - **SigV4**: standard AWS IAM credentials (access key + secret).
/// - **Bearer token**: an opaque token from an API Gateway fronting Bedrock or
///   a Vault-managed key dispensary.
///
/// # Examples
///
/// ```rust
/// use codetether_bedrock_auth::{AwsCredentials, BedrockAuth};
///
/// let sigv4 = BedrockAuth::SigV4(AwsCredentials {
///     access_key_id: "AKIA...".into(),
///     secret_access_key: "secret".into(),
///     session_token: None,
/// });
/// let bearer = BedrockAuth::bearer("token-abc".into());
///
/// match sigv4 {
///     BedrockAuth::SigV4(_) => (),
///     BedrockAuth::BearerToken(_) => panic!("unexpected"),
/// }
/// assert_eq!(bearer.current_bearer().as_deref(), Some("token-abc"));
/// ```
#[derive(Debug, Clone)]
pub enum BedrockAuth {
    /// Standard AWS SigV4 signing with IAM credentials.
    SigV4(AwsCredentials),
    /// Bearer token (API Gateway or custom auth layer).
    ///
    /// Held behind an an `Arc<RwLock<String>>` so a live provider can swap a
    /// refreshed token in place (see [`BedrockAuth::set_bearer`]) without
    /// rebuilding the provider — enabling mid-session SSO key refresh.
    BearerToken(std::sync::Arc<parking_lot::RwLock<String>>),
}
