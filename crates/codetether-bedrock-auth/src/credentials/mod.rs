//! Credential fields and environment/file lookup.

/// AWS credentials used for SigV4 signing of Bedrock requests.
///
/// Typically constructed via [`AwsCredentials::from_environment`] rather than
/// directly, so that env vars and the shared credentials file are both
/// respected.
///
/// # Examples
///
/// ```rust
/// use codetether_bedrock_auth::AwsCredentials;
///
/// let creds = AwsCredentials {
///     access_key_id: "AKIA...".to_string(),
///     secret_access_key: "secret".to_string(),
///     session_token: None,
/// };
/// assert_eq!(creds.access_key_id, "AKIA...");
/// assert!(creds.session_token.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct AwsCredentials {
    /// AWS access key ID (e.g. `AKIA...`).
    pub access_key_id: String,
    /// AWS secret access key (keep confidential).
    pub secret_access_key: String,
    /// Optional session token (present for STS / assumed-role credentials).
    pub session_token: Option<String>,
}
mod environment;
mod file;
mod region;
