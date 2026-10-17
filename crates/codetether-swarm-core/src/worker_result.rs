//! Swarm result text formatting.

/// Preserve nonblank swarm output or supply a completion-status fallback.
///
/// # Arguments
/// * `result` - Original textual output.
/// * `success` - Whether the swarm completed successfully.
/// # Returns
/// Original nonblank text, or the corresponding fallback message.
/// # Examples
/// ```rust
/// use codetether_swarm_core::worker_result::swarm_result_text;
/// assert_eq!(swarm_result_text("done", true), "done");
/// ```
pub fn swarm_result_text(result: &str, success: bool) -> String {
    if !result.trim().is_empty() {
        return result.to_string();
    }
    if success {
        "Swarm completed without textual output.".into()
    } else {
        "Swarm finished with failures and no textual output.".into()
    }
}
