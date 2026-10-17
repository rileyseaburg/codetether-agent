//! Gemini Web cookie handling, token fetching, and session cache policy.
//!
//! [GeminiWebSession] borrows the provider's existing client, cookie string,
//! and cache. [SessionTokens] holds scraped page credentials and account routing.
use anyhow::{Context, Result};
use regex::Regex;
use reqwest::Client;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
mod tokens;
pub use tokens::SessionTokens;
/// Origin used for Gemini page and request transport.
pub const GEMINI_ORIGIN: &str = "https://gemini.google.com";

/// How long cached session tokens remain valid before re-scraping the home page.
const TOKEN_TTL: Duration = Duration::from_secs(20 * 60);
/// Borrowed resources for one provider's Gemini Web session.
///
/// # Examples
/// ```
/// use codetether_gemini_web_transport::GeminiWebSession;
/// let client = reqwest::Client::new();
/// let cache = tokio::sync::Mutex::new(None);
/// let session = GeminiWebSession::new(&client, "", &cache);
/// assert!(session.cookie_header().is_empty());
/// ```
pub struct GeminiWebSession<'a> {
    client: &'a Client,
    cookies: &'a str,
    token_cache: &'a Mutex<Option<(SessionTokens, Instant)>>,
}
impl<'a> GeminiWebSession<'a> {
    /// Borrow an existing client, cookie string, and shared token cache.
    pub fn new(
        client: &'a Client,
        cookies: &'a str,
        token_cache: &'a Mutex<Option<(SessionTokens, Instant)>>,
    ) -> Self {
        Self {
            client,
            cookies,
            token_cache,
        }
    }
}
mod cookie_header;
mod get_or_refresh_tokens;
mod get_session_tokens;
mod invalidate_tokens;
mod parse_tokens;

use codetether_gemini_web_protocol::GeminiWebProtocol;

const GEMINI_STREAM_PATH: &str =
    "/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate";
mod ask;
mod build_request;
mod request_metadata;
mod send;

mod read_response;
mod request_url;
