//! Terminal prompt result type.

use serde::{Deserialize, Serialize};

/// Result returned from [`Session::prompt`](crate::session::Session::prompt)
/// and related prompt entry points.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::SessionResult;
///
/// let result = SessionResult {
///     text: "done".into(),
///     session_id: "session-1".into(),
/// };
/// assert_eq!(result.text, "done");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    /// Final assistant text answer.
    pub text: String,
    /// UUID of the session that produced the answer.
    pub session_id: String,
}
