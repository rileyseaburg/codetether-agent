//! HTTP request execution using a borrowed client and session routing state.
//!
//! The provider supplies payloads and credentials; this module sets transport
//! headers, sends each request, and captures successful routing responses.
use crate::{TurnStateStore, X_CODEX_TURN_STATE_HEADER, endpoints::*};
use anyhow::{Context, Result};
use serde_json::Value;
/// Borrowed HTTP resources for one provider's request execution.
///
/// # Examples
/// ```
/// let client = reqwest::Client::new();
/// let states = codetether_codex_transport::TurnStateStore::default();
/// let transport = codetether_codex_transport::HttpTransport::new(&client, &states);
/// ```
pub struct HttpTransport<'a> {
    client: &'a reqwest::Client,
    turn_states: &'a TurnStateStore,
}
impl<'a> HttpTransport<'a> {
    /// Borrow the provider's existing HTTP client and routing store.
    pub fn new(client: &'a reqwest::Client, turn_states: &'a TurnStateStore) -> Self {
        Self {
            client,
            turn_states,
        }
    }
}
mod chatgpt;
mod openai;
