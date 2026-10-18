//! Error markers for transient disconnections and permanent failures.
//! Permanent markers take precedence when both groups occur.
//! Stream-disconnect error classification.
//!
//! Distinguishes transient mid-stream disconnects (broken pipe, premature EOF,
//! h2/network resets) from permanent provider errors. Only transient disconnects
//! are eligible for same-model session auto-renew.

pub(crate) const DISCONNECT_MARKERS: &[&str] = &[
    "premature eof",
    "unexpected eof",
    "broken pipe",
    "connection reset",
    "connection closed",
    "stream closed",
    "h2 protocol error",
    "h2 error",
    "hyper: connection",
    "network error",
    "transport error",
    "tls handshake",
    "incomplete message",
    "body read",
    "end of stream",
];

pub(crate) const PERMANENT_MARKERS: &[&str] = &[
    "retry limit exhausted",
    "401",
    "403",
    "invalid api key",
    "unauthorized",
    "context length",
    "context_length",
    "maximum context",
    "content filter",
    "content_policy",
];
