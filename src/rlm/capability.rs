//! Per-tool output capability classification (issue #231 item 6).
//!
//! Consulted by [`crate::rlm::RlmRouter::should_route`] before any
//! routing decision. Only [`OutputCapability::BulkSummarizable`] tools
//! are ever destructively summarized; everything else is left intact.

use std::collections::HashSet;

/// What RLM is allowed to do with a tool's output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputCapability {
    /// Bulk web/aggregate output where a prose summary is an
    /// acceptable lossy projection.
    BulkSummarizable,
    /// Content-carrying output whose exact bytes may be needed later.
    ExactContent,
    /// Unknown tool. Treated as [`OutputCapability::ExactContent`].
    Unknown,
}

/// Tools eligible for destructive RLM routing (see module docs).
pub(super) fn rlm_eligible_tools() -> &'static HashSet<&'static str> {
    static TOOLS: std::sync::OnceLock<HashSet<&'static str>> = std::sync::OnceLock::new();
    TOOLS.get_or_init(|| ["webfetch", "websearch", "batch"].into_iter().collect())
}

/// Tools whose output carries exact bytes the agent may need later.
fn rlm_exact_content_tools() -> &'static HashSet<&'static str> {
    static TOOLS: std::sync::OnceLock<HashSet<&'static str>> = std::sync::OnceLock::new();
    TOOLS.get_or_init(|| {
        [
            "read",
            "grep",
            "bash",
            "glob",
            "ls",
            "edit",
            "write",
            "session_recall",
            "notebook_read",
            "notebook_edit",
        ]
        .into_iter()
        .collect()
    })
}

/// Classify a tool by name. `session_context` is the pseudo-tool used
/// by the compression pass and is the only summary-producing caller.
pub fn output_capability(tool_id: &str) -> OutputCapability {
    if tool_id == "session_context" || rlm_eligible_tools().contains(tool_id) {
        OutputCapability::BulkSummarizable
    } else if rlm_exact_content_tools().contains(tool_id) {
        OutputCapability::ExactContent
    } else {
        OutputCapability::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::{OutputCapability, output_capability};

    #[test]
    fn classifies_known_tools() {
        assert_eq!(
            output_capability("webfetch"),
            OutputCapability::BulkSummarizable
        );
        assert_eq!(
            output_capability("session_context"),
            OutputCapability::BulkSummarizable
        );
        assert_eq!(output_capability("read"), OutputCapability::ExactContent);
        assert_eq!(output_capability("bash"), OutputCapability::ExactContent);
        assert_eq!(
            output_capability("session_recall"),
            OutputCapability::ExactContent
        );
        assert_eq!(
            output_capability("brand_new_tool"),
            OutputCapability::Unknown
        );
    }
}
