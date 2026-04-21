//! Prompt builder for the LLM-based search router.
//!
//! Produces the single user-turn prompt that instructs the router model
//! to pick one (or more) backends for a user query and return structured
//! JSON. Router must emit JSON only — no markdown.

/// System message for the router model.
pub const ROUTER_SYSTEM: &str = "You are a search router. Given a user query, pick the best \
    search backend(s) and return ONLY JSON. No prose, no markdown.";

/// Build the user prompt.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::search::prompt::build_user_prompt;
/// let p = build_user_prompt("how to center a div", 1);
/// assert!(p.contains("websearch"));
/// assert!(p.contains("how to center a div"));
/// ```
pub fn build_user_prompt(query: &str, top_n: usize) -> String {
    format!(
        "Query:\n{query}\n\n\
         Available backends:\n\
         - grep: exact or regex text search across the workspace (args: {{\"pattern\": string, \"is_regex\"?: bool, \"path\"?: string, \"limit\"?: int}})\n\
         - glob: filename glob search (args: {{\"pattern\": string}})\n\
         - websearch: live web search (args: {{\"query\": string, \"max_results\"?: int}})\n\
         - webfetch: fetch a specific URL (args: {{\"url\": string}})\n\
         - memory: search persisted agent memory (args: {{\"action\": \"search\", \"query\": string}})\n\
         - rlm: semantic analysis across large codebases (args: {{\"action\": \"search\", \"query\": string}})\n\n\
         Return JSON ONLY in this exact shape:\n\
         {{\"choices\":[{{\"backend\":\"grep|glob|websearch|webfetch|memory|rlm\",\"args\":{{...}}}}]}}\n\
         Pick at most {top_n} choice(s). Order by best first."
    )
}
