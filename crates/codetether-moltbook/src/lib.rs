//! Moltbook API client, wire types, and CodeTether content helpers.
//!
//! [MoltbookClient] handles credentials and remote operations. Wire types model
//! registration, profiles, posts, and API responses. [intro_post] formats an
//! introduction for callers to inspect before publishing.
mod build_codetether_description;
mod client;
mod intro_post;
mod wire;
use build_codetether_description::build_codetether_description;
pub use client::MoltbookClient;
pub use intro_post::intro_post;
pub use wire::*;

/// Moltbook API base — ALWAYS use www to avoid redirect-stripping the auth header.
const API_BASE: &str = "https://www.moltbook.com/api/v1";

/// Vault path where the Moltbook API key is stored.
const VAULT_PROVIDER_ID: &str = "moltbook";
#[cfg(test)]
mod tests;
