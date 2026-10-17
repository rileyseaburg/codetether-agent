//! Tool-output capability classification used by [`crate::should_route`].

mod kind;
mod registry;
pub use kind::OutputCapability;
pub(super) use registry::rlm_eligible_tools;
use registry::rlm_exact_content_tools;

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
mod tests;
