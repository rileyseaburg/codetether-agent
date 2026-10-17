//! Project belief context operation.
use crate::beliefs::{Belief, BeliefStatus};

/// Build a system-context string from high-confidence beliefs.
/// Only includes beliefs with confidence >= 0.7 and Active status.
///
/// # Arguments
/// * `beliefs` - Stored beliefs in the desired presentation order.
///
/// # Returns
/// Up to twenty active claims, or empty text when none qualify.
///
/// # Examples
/// ```
/// use codetether_cognition_core::palace::belief_context;
/// assert!(belief_context(&[]).is_empty());
/// ```
pub fn belief_context(beliefs: &[Belief]) -> String {
    let active: Vec<&Belief> = beliefs
        .iter()
        .filter(|b| b.confidence >= 0.7 && b.status == BeliefStatus::Active)
        .take(20)
        .collect();
    if active.is_empty() {
        return String::new();
    }
    let mut ctx = String::from("Project knowledge:\n");
    for b in &active {
        ctx.push_str(&format!("- {} (conf: {:.2})\n", b.claim, b.confidence));
    }
    ctx
}
