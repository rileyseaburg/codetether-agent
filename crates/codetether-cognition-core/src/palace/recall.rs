//! Recall-command filtering and presentation for project memories.
use crate::beliefs::Belief;
/// Render up to thirty stored beliefs with confidence at least one half.
///
/// # Arguments
/// * `beliefs` — Stored project memories in their existing order.
/// # Returns
/// The recall report, including the existing empty-memory guidance.
/// # Examples
/// ```
/// assert!(codetether_cognition_core::palace::recall_message(&[]).contains("No project memories"));
/// ```
pub fn recall_message(beliefs: &[Belief]) -> String {
    if beliefs.is_empty() {
        return "No project memories found. Memories accumulate as you work.".to_string();
    }
    let active: Vec<&Belief> = beliefs
        .iter()
        .filter(|b| b.confidence >= 0.5)
        .take(30)
        .collect();
    let mut lines = vec![format!(
        "🏰 Project Memory Palace ({} beliefs, {} active)",
        beliefs.len(),
        active.len()
    )];
    for b in &active {
        lines.push(format!("  • {} [conf: {:.2}]", b.claim, b.confidence));
    }
    lines.join("\n")
}
