//! Relay context context distillation query.

/// Build the structured RLM query for a relay turn.
///
/// # Arguments
/// * `task` - Overall task to retain in context.
/// * `from_agent` - Agent whose output is being distilled.
///
/// # Returns
/// The complete query requesting a structured context delta.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::context_distillation_query;
/// assert!(context_distillation_query("review", "worker").contains("review"));
/// ```
pub fn context_distillation_query(task: &str, from_agent: &str) -> String {
    format!(
        "Task:\n{task}\n\nAgent turn source: @{from_agent}\n\n\
Distill this turn into shared relay context.\n\
Return FINAL(JSON) with this shape:\n\
{{\"kind\":\"semantic\",\"file\":\"relay_context_delta\",\"answer\":\"{{\\\"facts\\\":[\\\"...\\\"],\\\"decisions\\\":[\\\"...\\\"],\\\"risks\\\":[\\\"...\\\"],\\\"next_actions\\\":[\\\"...\\\"],\\\"evidence\\\":[\\\"...\\\"]}}\"}}\n\
Rules:\n\
- answer MUST be valid JSON object encoded as a string\n\
- 0-3 items per field\n\
- no markdown"
    )
}
