//! [`SpeechAct`] string conversions and reply semantics.

use super::SpeechAct;

impl SpeechAct {
    /// Canonical lowercase wire string for this act.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::Inform => "inform",
            Self::Propose => "propose",
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Claim => "claim",
            Self::Blocked => "blocked",
            Self::Done => "done",
            Self::Say => "say",
        }
    }

    /// Parse leniently (case-insensitive); unknown verbs map to [`Self::Say`].
    pub fn from_str_lenient(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "request" | "ask" => Self::Request,
            "inform" | "info" => Self::Inform,
            "propose" | "suggest" => Self::Propose,
            "accept" | "agree" => Self::Accept,
            "reject" | "deny" | "disagree" => Self::Reject,
            "claim" => Self::Claim,
            "blocked" | "block" => Self::Blocked,
            "done" | "complete" | "completed" => Self::Done,
            _ => Self::Say,
        }
    }

    /// Whether the sender typically expects a reply for this act.
    pub fn expects_reply(&self) -> bool {
        matches!(self, Self::Request | Self::Propose | Self::Blocked)
    }
}
