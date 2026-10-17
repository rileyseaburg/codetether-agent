//! oauth expiry.
use crate::*;
impl CodexOAuth {
    /// Resolve a token lifetime against the current system clock.
    ///
    /// # Arguments
    /// * `expires_in` - Token lifetime in seconds.
    ///
    /// # Returns
    /// Expiration timestamp in Unix seconds.
    ///
    /// # Errors
    /// Returns an error if the clock precedes the Unix epoch.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_codex_auth::CodexOAuth::oauth_expiry(0).is_ok());
    /// ```
    pub fn oauth_expiry(expires_in: u64) -> Result<u64> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + expires_in)
    }
}
