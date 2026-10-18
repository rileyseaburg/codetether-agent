//! A peer's reply, independent of how it is shown to the model or the TUI.

use codetether_a2a_core::SendMessageResponse;

/// What a LAN peer said in response to one turn.
/// # Examples
/// ```rust
/// let reply = codetether_peer_agent::reply::PeerReply::ok("done");
/// assert!(!reply.failed);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerReply {
    /// Plain text of the reply; never empty.
    pub text: String,
    /// Whether the peer reported the turn as failed.
    pub failed: bool,
}

impl PeerReply {
    /// A successful plain-text reply.
    pub fn ok(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            failed: false,
        }
    }

    /// Extract response text and failure state, supplying the existing empty-text fallback.
    pub fn from_response(response: &SendMessageResponse) -> Self {
        let (text, failed) = super::text::response(response);
        let text = match text.trim() {
            "" => "Peer completed without a text response".to_string(),
            _ => text,
        };
        Self { text, failed }
    }

    /// Format the existing transport-failure response.
    pub fn transport_failure(error: &str) -> Self {
        Self {
            text: format!("Remote call failed: {error}"),
            failed: true,
        }
    }
}
