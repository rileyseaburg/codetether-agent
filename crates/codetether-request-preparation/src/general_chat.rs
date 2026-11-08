//! General conversations have no implicit repository or project-task ledger.
use codetether_provider_core::ToolDefinition;
use std::path::PathBuf;

/// Select an explicit chat session or a dedicated chat-only server process.
/// # Arguments
/// `agent` is the requested session persona.
/// # Returns
/// True for `chat` or `CODETETHER_CHAT_MODE=general`.
/// # Examples
/// ```
/// assert!(codetether_request_preparation::general_chat::enabled("chat"));
/// ```
pub fn enabled(agent: &str) -> bool {
    agent == "chat" || std::env::var("CODETETHER_CHAT_MODE").is_ok_and(|v| v == "general")
}
/// Resolve neutral scratch storage, not a persisted project working directory.
/// # Returns
/// Workspace, disabled cross-project history, and disabled autonomous mode.
/// # Examples
/// ```
/// let (_, prior, autonomous) = codetether_request_preparation::general_chat::context();
/// assert!(!prior && !autonomous);
/// ```
pub fn context() -> (PathBuf, bool, bool) {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"));
    let fallback = home
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join(".local/share/codetether-chat");
    let cwd = std::env::var_os("CODETETHER_CHAT_WORKSPACE")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or(fallback);
    (cwd, false, false)
}
/// Build a general-purpose prompt without reading repository instructions.
/// # Arguments
/// `native` selects native tools; `tools` supplies textual-tool schemas.
/// # Returns
/// General conversation instructions with optional tool-call markup.
/// # Examples
/// ```
/// assert!(codetether_request_preparation::general_chat::prompt(true, &[]).contains("general-purpose"));
/// ```
pub fn prompt(native: bool, tools: &[ToolDefinition]) -> String {
    let base = "You are CodeTether Chat, a general-purpose conversational assistant. There is no selected project or repository. Follow the user's current request; do not assume a coding task, resume old project work, or inspect local projects unless explicitly asked. Earlier project references are historical context, not an active assignment. Use available tools, including web research, when helpful; discover additional tools with list_tools. Never invent tool results.";
    if native || tools.is_empty() {
        base.into()
    } else {
        codetether_tool_catalog::inject_tool_prompt(base, tools)
    }
}
