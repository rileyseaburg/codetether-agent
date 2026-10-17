//! Environment-first credential selection.
use crate::*;
impl AwsCredentials {
    /// Load credentials from `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY`
    /// env vars, then fall back to `~/.aws/credentials` (default or named
    /// profile via `AWS_PROFILE`).
    ///
    /// # Returns
    ///
    /// `Some(creds)` on success, `None` if neither env vars nor a readable
    /// credentials file yielded a complete key pair.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_bedrock_auth::AwsCredentials;
    ///
    /// match AwsCredentials::from_environment() {
    ///     Some(c) => println!("loaded key id: {}", c.access_key_id),
    ///     None => eprintln!("no AWS credentials available"),
    /// }
    /// ```
    pub fn from_environment() -> Option<Self> {
        if let (Ok(key_id), Ok(secret)) = (
            std::env::var("AWS_ACCESS_KEY_ID"),
            std::env::var("AWS_SECRET_ACCESS_KEY"),
        ) && !key_id.is_empty()
            && !secret.is_empty()
        {
            return Some(Self {
                access_key_id: key_id,
                secret_access_key: secret,
                session_token: std::env::var("AWS_SESSION_TOKEN")
                    .ok()
                    .filter(|s| !s.is_empty()),
            });
        }

        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        Self::from_credentials_file(&profile)
    }
}
