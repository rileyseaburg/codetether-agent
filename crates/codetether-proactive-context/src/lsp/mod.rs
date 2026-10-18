//! Prefetch diagnostics and symbol context before a session's first response.
//!
//! [build] resolves candidate paths from the latest user message, asks the supplied
//! LSP tool for diagnostics, and falls back to document symbols per file.
mod diagnostics;
mod message;
mod settings;
mod symbols;
use codetether_build_policy::extract_candidate_file_paths;
use codetether_context_policy::transcript::latest_user_text;
use codetether_provider_core::Message;
use codetether_tool_core::Tool;
use std::{path::Path, sync::Arc};
/// Gather first-turn LSP context using the application's registered tool.
/// # Arguments
/// * `step` — Session step, with collection enabled only for step one.
/// * `lookup` — Deferred lookup of the registered LSP tool.
/// * `session_messages` — Conversation used to locate the latest user request.
/// * `workspace_dir` — Root for candidate file resolution.
/// # Returns
/// A system message with useful context, or None when no context is available.
/// Tool failures are logged and may trigger symbol fallback.
/// # Examples
/// ```no_run
/// # async fn example() {
/// assert!(codetether_proactive_context::lsp::build(2, || None, &[], std::path::Path::new(".")).await.is_none());
/// # }
/// ```
pub async fn build(
    step: usize,
    lookup: impl FnOnce() -> Option<Arc<dyn Tool>>,
    session_messages: &[Message],
    workspace_dir: &Path,
) -> Option<Message> {
    if step != 1 {
        return None;
    }

    let Some(lsp_tool) = lookup() else {
        return None;
    };

    let Some(user_text) = latest_user_text(session_messages) else {
        return None;
    };

    let (max_files, max_chars) = settings::limits();

    let paths = extract_candidate_file_paths(&user_text, workspace_dir, max_files);
    if paths.is_empty() {
        return None;
    }

    let mut sections = Vec::new();
    for path in paths {
        if diagnostics::append(lsp_tool.as_ref(), &path, max_chars, &mut sections).await {
            continue;
        }
        symbols::append(lsp_tool.as_ref(), &path, max_chars, &mut sections).await;
    }
    message::from_sections(sections)
}
