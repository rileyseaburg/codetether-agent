//! Relay context fallback context delta.
use super::*;

/// Preserve a bounded output excerpt when structured distillation is unavailable.
///
/// # Arguments
/// * `output` - Original agent output.
/// * `from_agent` - Agent identity to retain in the fact.
///
/// # Returns
/// A delta with a fact, next action and output-length evidence.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::fallback_context_delta;
/// assert!(!fallback_context_delta("done", "worker").facts.is_empty());
/// ```
pub fn fallback_context_delta(output: &str, from_agent: &str) -> ContextDelta {
    let excerpt = truncate_chars(output.trim(), 240);
    let fact = if excerpt.is_empty() {
        format!("@{from_agent} produced an empty turn output")
    } else {
        format!("@{from_agent}: {excerpt}")
    };

    ContextDelta {
        facts: vec![fact],
        decisions: Vec::new(),
        risks: Vec::new(),
        next_actions: vec!["Continue with one concrete implementation step.".to_string()],
        evidence: vec![format!("output_chars={}", output.chars().count())],
    }
}
