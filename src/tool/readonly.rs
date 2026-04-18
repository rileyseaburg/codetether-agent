//! Read-only tool allowlist for parallel dispatch.
//!
//! When a provider emits multiple tool calls in a single assistant turn
//! (e.g. OpenAI parallel_tool_calls, Claude parallel tools), independent
//! read-only calls can be executed concurrently instead of sequentially.
//! This module maintains the hardcoded allowlist of tool IDs known to be
//! side-effect-free.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::tool::readonly::is_read_only;
//!
//! assert!(is_read_only("read"));
//! assert!(is_read_only("grep"));
//! assert!(!is_read_only("bash"));
//! assert!(!is_read_only("write"));
//! ```

/// Tool IDs whose `execute()` is guaranteed not to mutate the filesystem,
/// network-visible resources, or internal agent state.
const READ_ONLY_TOOL_IDS: &[&str] = &[
    "read",
    "list",
    "glob",
    "tree",
    "fileinfo",
    "headtail",
    "diff",
    "grep",
    "codesearch",
    "lsp",
    "webfetch",
    "websearch",
];

/// Returns `true` if `tool_id` identifies a known read-only tool that is
/// safe to dispatch concurrently with other read-only tool calls.
///
/// Unknown or side-effecting tools return `false` and must be executed
/// sequentially to preserve ordering semantics.
pub fn is_read_only(tool_id: &str) -> bool {
    READ_ONLY_TOOL_IDS.contains(&tool_id)
}
