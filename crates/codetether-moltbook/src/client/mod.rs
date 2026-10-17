//! Client state with credential, API operation, and HTTP helper modules.
//!
//! Construct [MoltbookClient] with a key or load it through its credential helper.
use super::*;
use anyhow::{Context, Result};
mod claim_status;
mod comment;
mod create_post;
mod feed;
mod from_vault_or_env;
mod get;
mod heartbeat;
mod me;
mod new;
mod patch;
mod post_json;
mod register;
mod save_key_to_vault;
mod search;
mod update_profile;
mod upvote;
mod validate_url;

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// Secure Moltbook client.
///
/// The API key is:
///   1. loaded from Vault (`codetether/moltbook`)
///   2. OR from the `MOLTBOOK_API_KEY` env var (fallback for local dev)
///
/// The key is ONLY ever sent to `https://www.moltbook.com`.
///
/// # Examples
/// ```no_run
/// let client = codetether_moltbook::MoltbookClient::new("example".into());
/// let _ = client;
/// ```
pub struct MoltbookClient {
    pub(super) http: reqwest::Client,
    pub(super) api_key: String,
}

#[cfg(test)]
mod tests;
