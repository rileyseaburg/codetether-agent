//! Bind the reusable undo tool to the application's session-aware policy.
pub use codetether_undo_tool::UndoInput;
/// Undo tool using the application's authoritative runtime policy.
///
/// # Examples
/// ```rust
/// use codetether_native_edit::undo::UndoTool;
/// let _tool: UndoTool = UndoTool;
/// ```
pub type UndoTool = codetether_undo_tool::Undo<ApplicationPolicy>;
/// Ready-to-register undo tool; preserves the existing unit-like construction.
///
/// # Examples
/// ```rust
/// let _tool = codetether_native_edit::undo::UndoTool;
/// ```
#[allow(non_upper_case_globals)]
pub const UndoTool: UndoTool = codetether_undo_tool::Undo::new();
/// Application adapter for undo authorization.
///
/// # Examples
/// ```rust
/// let _policy=codetether_native_edit::undo::ApplicationPolicy;
/// ```
pub struct ApplicationPolicy;
#[async_trait::async_trait]
impl codetether_undo_tool::Policy for ApplicationPolicy {
    async fn blocked(args: &serde_json::Value) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation("undo", args).await
    }
}
