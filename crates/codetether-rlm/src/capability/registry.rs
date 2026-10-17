//! Stable classification sets for tool outputs.

use std::collections::HashSet;

/// Tools eligible for destructive RLM routing (see module docs).
pub(crate) fn rlm_eligible_tools() -> &'static HashSet<&'static str> {
    static TOOLS: std::sync::OnceLock<HashSet<&'static str>> = std::sync::OnceLock::new();
    TOOLS.get_or_init(|| ["webfetch", "websearch", "batch"].into_iter().collect())
}

/// Tools whose output carries exact bytes the agent may need later.
pub(super) fn rlm_exact_content_tools() -> &'static HashSet<&'static str> {
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
