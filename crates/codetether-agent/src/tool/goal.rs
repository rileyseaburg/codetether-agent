//! First-class persisted goal tools plus the legacy session-task surface.

#[path = "goal/attempt_cap.rs"]
pub mod attempt_cap;
pub use codetether_native_goal_tools::verdict_log;
#[path = "goal/verify.rs"]
pub mod verify;

use super::ToolRegistry;
use codetether_tool_registry::catalog::ToolKind;
/// Construct a native goal tool using the application's existing verifier.
pub(crate) fn construct(kind: ToolKind) -> Option<std::sync::Arc<dyn super::Tool>> {
    codetether_native_goal_tools::catalog::construct::<verify::LlmVerifier>(kind)
}

/// Register goal lifecycle tools and the backward-compatible task tool.
pub fn register(registry: &mut ToolRegistry) {
    super::catalog::register_group(registry, ToolKind::GoalGroup);
}
