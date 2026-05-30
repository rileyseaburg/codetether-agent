//! Swarm result text formatting.

pub(super) fn swarm_result_text(result: &str, success: bool) -> String {
    if !result.trim().is_empty() { return result.to_string(); }
    if success { "Swarm completed without textual output.".into() }
    else { "Swarm finished with failures and no textual output.".into() }
}
