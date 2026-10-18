//! Shared user authentication load operations.

use super::path::credential_file_path;
use crate::SavedCredentials;

/// Load saved credentials from disk, returning `None` if the file doesn't exist,
/// is malformed, or a parseable expiry is in the past.
///
/// # Returns
/// A saved record, or None when lookup, decoding or expiry validation fails.
/// Unparseable expiry strings retain the existing permissive behavior.
/// # Examples
/// ```no_run
/// let credentials = codetether_user_auth::load_saved_credentials();
/// ```
pub fn load_saved_credentials() -> Option<SavedCredentials> {
    let path = credential_file_path().ok()?;
    let data = std::fs::read_to_string(&path).ok()?;
    let creds: SavedCredentials = serde_json::from_str(&data).ok()?;

    // Check expiry if parseable
    if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&creds.expires_at)
        && expires < chrono::Utc::now()
    {
        tracing::warn!("Saved credentials have expired — run `codetether auth login` to refresh");
        return None;
    }

    Some(creds)
}
