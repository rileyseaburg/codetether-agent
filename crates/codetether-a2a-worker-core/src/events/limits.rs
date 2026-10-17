//! Structured event byte limits and sensitive-key fragments.

/// Maximum bytes of tool output carried in one event payload.
///
/// Generous enough for a real `bash` result or diff while still bounding the
/// events table. Truncation is always recorded explicitly rather than silently
/// clipping, so a reader can tell a short result from a cut-off one.
pub const MAX_OUTPUT_BYTES: usize = 8192;

/// Maximum bytes of a single serialized tool argument value.
pub const MAX_ARG_BYTES: usize = 2048;

/// Argument names whose values must never be published.
///
/// Structured arguments are a new exposure surface: a flattened 500-byte string
/// often hid a credential that a full `bash` command or `write` body would now
/// reveal. Over-redacting is preferred to leaking a token onto an issue page.
pub(super) const SENSITIVE_KEYS: &[&str] = &[
    "token",
    "secret",
    "password",
    "passwd",
    "authorization",
    "api_key",
    "apikey",
    "access_key",
    "private_key",
    "credential",
    "credentials",
    "session_token",
    "bearer",
];
