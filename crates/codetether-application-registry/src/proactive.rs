//! Proactive LSP context using the application's registered tool instance.
//!
//! [`build_proactive_lsp_context_message`] binds registry lookup to the shared
//! LSP context policy. The policy selects when to request diagnostics and
//! constructs the message using the caller's workspace and transcript.
use crate::{Application, ToolRegistry};
use codetether_provider_core::Message;
use std::path::Path;

/// Build optional proactive diagnostics using the registered LSP tool.
/// # Arguments
/// `selected_provider` is retained for caller compatibility; diagnostics policy
/// uses `step`, `session_messages` and `workspace_dir`. `tool_registry` supplies
/// the existing LSP tool without constructing a replacement.
/// # Returns
/// A diagnostics context message when the shared policy chooses one.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_registry::Application>(registry: &codetether_application_registry::ToolRegistry<H>) {
/// let _context = codetether_application_registry::proactive::build_proactive_lsp_context_message(
///     "provider", 1, registry, &[], std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn build_proactive_lsp_context_message<H: Application>(
    _selected_provider: &str,
    step: usize,
    tool_registry: &ToolRegistry<H>,
    session_messages: &[Message],
    workspace_dir: &Path,
) -> Option<Message> {
    codetether_proactive_context::lsp::build(
        step,
        || tool_registry.get("lsp"),
        session_messages,
        workspace_dir,
    )
    .await
}
