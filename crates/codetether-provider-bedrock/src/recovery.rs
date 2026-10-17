//! Application-supplied credential refresh policy.
use async_trait::async_trait;
use reqwest::StatusCode;
/// Credential recovery invoked after an HTTP authentication failure.
///
/// # Implementors
/// Applications implement this interface to refresh the same shared auth state
/// borrowed by [crate::BedrockClient]. The client controls when recovery is attempted.
///
/// # Examples
/// ```
/// struct NoRecovery;
/// #[async_trait::async_trait]
/// impl codetether_provider_bedrock::recovery::CredentialRecovery for NoRecovery {
///     async fn recover(&self) -> bool { false }
/// }
/// ```
#[async_trait]
pub trait CredentialRecovery: Send + Sync {
    /// Return true after installing credentials that warrant retrying the request.
    async fn recover(&self) -> bool;
}
/// Whether the service rejected the request's credentials.
///
/// # Arguments
/// * `status` - HTTP response status.
///
/// # Returns
/// True for HTTP 401 or 403.
///
/// # Examples
/// ```
/// assert!(codetether_provider_bedrock::recovery::is_auth_failure(reqwest::StatusCode::UNAUTHORIZED));
/// ```

/// True for the statuses Bedrock uses to reject an expired/invalid key.
pub fn is_auth_failure(status: StatusCode) -> bool {
    matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
}
