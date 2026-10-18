//! Track repeated edit attempts for one prompt turn.
use super::{
    limits::{EDIT_TOOLS, THRESHOLD},
    signature::signature,
};
use serde_json::Value;
use std::collections::HashMap;

/// Tracks per-signature call counts for the duration of one agentic turn.
/// # Examples
/// ```
/// use codetether_repeat_guard::RepeatGuard;
/// let mut guard = RepeatGuard::default();
/// assert!(guard.check("read", &serde_json::json!({"path": "file"})).is_none());
/// ```
#[derive(Default)]
pub struct RepeatGuard {
    counts: HashMap<String, u8>,
}

impl RepeatGuard {
    /// Returns `Some(reason)` when the tool call should be blocked.
    /// # Arguments
    /// Receives a tool name and parsed input; every guarded call counts.
    /// # Returns
    /// A rejection message after three attempts with the same canonical signature.
    /// # Panics
    /// In builds with overflow checks, panics if one signature's u8 counter overflows.
    /// # Examples
    /// ```
    /// let mut guard = codetether_repeat_guard::RepeatGuard::default();
    /// let input = serde_json::json!({"path": "a.rs", "new_string": "x"});
    /// for _ in 0..3 { assert!(guard.check("write", &input).is_none()); }
    /// assert!(guard.check("write", &input).is_some());
    /// ```
    pub fn check(&mut self, tool_name: &str, tool_input: &Value) -> Option<String> {
        if !EDIT_TOOLS.contains(&tool_name) {
            return None;
        }
        let sig = signature(tool_name, tool_input);
        let count = self.counts.entry(sig).or_insert(0);
        *count += 1;
        (*count > THRESHOLD).then(|| {
            "You have already attempted this exact edit multiple times and it was \
             not applied. Stop retrying the identical edit — the target text may \
             have changed or the edit may not be applicable. Re-read the file to \
             see its current content, then adjust your edit or report the blocker."
                .to_string()
        })
    }
}
