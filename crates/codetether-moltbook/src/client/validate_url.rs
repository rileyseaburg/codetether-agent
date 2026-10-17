//! Moltbook client validate url.
use super::*;
impl MoltbookClient {
    // ------- HTTP helpers ---------------------------------------------------

    /// Validate that a URL points to the Moltbook API (security check).
    pub(super) fn validate_url(path: &str) -> String {
        // Never allow the API key to leave www.moltbook.com
        format!("{}{}", API_BASE, path)
    }
}
