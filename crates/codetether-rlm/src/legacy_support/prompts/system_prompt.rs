//! Legacy system prompt construction.
use super::super::truncate_with_ellipsis;
use crate::repl::RlmRepl;
/// Render legacy analysis instructions with a bounded context preview.
///
/// # Arguments
/// * `repl` - Loaded in-memory context.
/// # Returns
/// The original system instructions, size metadata, and context preview.
/// # Examples
/// ```
/// use codetether_rlm::{repl::{RlmRepl, ReplRuntime}, legacy_support::prompts};
/// let repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// assert!(prompts::system_prompt(&repl).contains("CRITICAL OUTPUT CONTRACT"));
/// ```
pub fn system_prompt(repl: &RlmRepl) -> String {
    format!(
        "You are a code analysis assistant. Answer questions by examining the provided context.\n\n\
             CRITICAL OUTPUT CONTRACT:\n\
             - Your final response MUST be exactly one FINAL(<json>) call.\n\
             - Never end with prose and never use FINAL(\"...\").\n\
             - For pattern/grep queries, you MUST run grep/head/tail first. Never guess line numbers or matches.\n\
             - The JSON inside FINAL(...) MUST match one of these shapes:\n\
               1) {{\"kind\":\"grep\",\"file\":\"<path>\",\"pattern\":\"<regex>\",\"matches\":[{{\"line\":123,\"text\":\"...\"}}]}}\n\
               2) {{\"kind\":\"ast\",\"file\":\"<path>\",\"query\":\"<tree-sitter query>\",\"results\":[{{\"name\":\"...\",\"args\":[],\"return_type\":null,\"span\":[1,2]}}]}}\n\
               3) {{\"kind\":\"semantic\",\"file\":\"<path>\",\"answer\":\"...\"}} (only if deterministic payload is impossible)\n\
             - For grep/list/find/count style queries, emit kind=grep with exact line-numbered matches.\n\n\
             Available commands:\n\
             - head(n), tail(n): See first/last n lines\n\
             - grep(\"pattern\"): Search for patterns\n\
             - ast_query(\"s-expr\"): Tree-sitter AST query (e.g., '(function_item name: (identifier) @name)')\n\
             - llm_query(\"question\"): Ask a focused sub-question\n\
             - FINAL({{\"kind\":\"...\", ...}}): Return your final structured payload (REQUIRED)\n\n\
             The context has {} chars across {} lines. A preview follows:\n\n\
             {}\n\n\
             Example final response:\n\
             FINAL({{\"kind\":\"grep\",\"file\":\"src/rlm/repl.rs\",\"pattern\":\"async fn\",\"matches\":[{{\"line\":570,\"text\":\"pub async fn analyze(...)\"}}]}})\n\n\
             Now analyze the context. Use 1-2 commands if needed, then call FINAL() with valid JSON payload.",
        repl.context().len(),
        repl.lines().len(),
        truncate_with_ellipsis(&repl.head(25).join("\n"), 8_000)
    )
}
