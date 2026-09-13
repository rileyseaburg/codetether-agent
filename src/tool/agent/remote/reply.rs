//! A peer's reply, independent of how it is shown to the model or the TUI.

use crate::a2a::types::SendMessageResponse;

/// What a LAN peer said in response to one turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tool::agent) struct PeerReply {
    /// Plain text of the reply; never empty.
    pub text: String,
    /// Whether the peer reported the turn as failed.
    pub failed: bool,
}

impl PeerReply {
    /// A successful plain-text reply.
    pub(in crate::tool::agent) fn ok(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            failed: false,
        }
    }

    pub(super) fn from_response(response: &SendMessageResponse) -> Self {
        let (text, failed) = super::text::response(response);
        let text = match text.trim() {
            "" => "Peer completed without a text response".to_string(),
            _ => text,
        };
        Self { text, failed }
    }

    pub(super) fn transport_failure(error: &str) -> Self {
        Self {
            text: format!("Remote call failed: {error}"),
            failed: true,
        }
    }
}
