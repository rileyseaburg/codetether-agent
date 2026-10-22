//! Content hashes exclude only top-level runtime routing metadata.
use sha2::{Digest, Sha256};
pub(super) fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
pub(super) fn signature(tool: &str, input: &serde_json::Value) -> String {
    let (tool, mut input) = codetether_edit_policy::normalize_tool_call_for_execution(tool, input);
    if let Some(map) = input.as_object_mut() {
        for key in ["__ct_session_id", "approval_id", "justification"] {
            map.remove(key);
        }
    }
    digest(&format!("{tool}:{}", input))
}
pub(super) fn waiting(tool: &str) -> bool {
    matches!(
        tool,
        "wait" | "sleep" | "poll" | "write_stdin" | "wait_agent" | "agent_wait"
    )
}
