//! Legacy context summary construction.
use crate::repl::RlmRepl;
/// Describe the context size and available legacy DSL operations.
///
/// # Arguments
/// * `repl` - Loaded in-memory context.
/// # Returns
/// The original human-readable context summary.
/// # Examples
/// ```
/// use codetether_rlm::{repl::{RlmRepl, ReplRuntime}, legacy_support::prompts};
/// let repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// assert!(prompts::context_summary(&repl).contains("4 chars"));
/// ```
pub fn context_summary(repl: &RlmRepl) -> String {
    format!(
        "=== CONTEXT LOADED ===\n\
             Total: {} chars, {} lines\n\
             Available functions:\n\
             - head(n) - first n lines\n\
             - tail(n) - last n lines\n\
             - grep(\"pattern\") - find lines matching regex\n\
             - count(\"pattern\") - count regex matches\n\
             - slice(start, end) - slice by char position\n\
             - chunks(n) - split into n chunks\n\
             - ast_query(\"s-expr\") - tree-sitter AST query for structural analysis\n\
             - llm_query(\"question\", context?) - ask sub-LM a question\n\
             - FINAL({{json_payload}}) - return structured final payload\n\
             === END CONTEXT INFO ===",
        repl.context().len(),
        repl.lines().len()
    )
}
