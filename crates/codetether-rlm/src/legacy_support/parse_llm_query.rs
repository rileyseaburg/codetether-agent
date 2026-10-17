//! Legacy executor parse llm query.
use super::*;

/// Parse llm_query("question", context?) call
/// Parse a sub-query and evaluate its optional context expression.
///
/// # Arguments
/// * `line` - Legacy llm_query call text.
/// * `repl` - Context evaluator used by the optional second argument.
/// # Returns
/// Question text and an optional evaluated context slice.
/// # Examples
/// ```
/// use codetether_rlm::repl::{RlmRepl, ReplRuntime};
/// let mut repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// assert_eq!(codetether_rlm::legacy_support::parse_llm_query("llm_query("question")", &mut repl).0, "question");
/// ```
/// # Panics
/// Malformed delimiters or invalid nested expressions may panic, matching the
/// legacy parser and REPL behavior.
pub fn parse_llm_query(line: &str, repl: &mut RlmRepl) -> (String, Option<String>) {
    // Find the query string
    let start = line.find('(').unwrap_or(0) + 1;
    let end = line.rfind(')').unwrap_or(line.len());
    let args = &line[start..end];

    // Split by comma, but respect quotes
    let mut query = String::new();
    let mut context = None;
    let mut in_quotes = false;
    let mut current = String::new();
    let mut parts = Vec::new();

    for c in args.chars() {
        if c == '"' || c == '\'' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            parts.push(current.trim().to_string());
            current = String::new();
            continue;
        }
        current.push(c);
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    // First part is the query
    if let Some(q) = parts.first() {
        query = q.trim_matches(|c| c == '"' || c == '\'').to_string();
    }

    // Second part (if present) is context expression
    if let Some(ctx_expr) = parts.get(1) {
        // Evaluate the context expression
        let ctx = repl.evaluate_expression(ctx_expr);
        if !ctx.is_empty() && !ctx.starts_with('[') {
            context = Some(ctx);
        }
    }

    (query, context)
}
