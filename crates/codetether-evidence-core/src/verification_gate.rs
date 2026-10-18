//! Check and rewrite unsupported verification assertions.
//! Final-response enforcement for verification evidence labels.

use super::gate_mode::GateMode;

/// Apply the selected evidence-label and source checks to answer text.
/// # Arguments
/// `mode` chooses enforcement; `answer` contains proposed final-response text.
/// # Returns
/// The original answer, a warning-appended answer, or rewritten claim lines.
/// # Examples
/// ```
/// use codetether_evidence_core::{gate_mode::GateMode, verification_gate::apply};
/// assert_eq!(apply(GateMode::Off, "answer"), "answer");
/// ```
pub fn apply(mode: GateMode, answer: &str) -> String {
    let violations = super::verification_claim::violations(answer);
    if violations.is_empty() || mode == GateMode::Off {
        return answer.to_string();
    }
    match mode {
        GateMode::Strict => rewrite(answer),
        GateMode::Warn => warn(answer, violations.len()),
        GateMode::Off => answer.to_string(),
    }
}

fn rewrite(answer: &str) -> String {
    answer
        .lines()
        .map(|line| {
            if super::verification_claim::is_valid(line) {
                line.to_string()
            } else if super::verification_claim::is_verification(line) {
                "[verification claim withheld: add an allowed validation level and concrete evidence source]".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn warn(answer: &str, count: usize) -> String {
    format!(
        "{answer}\n\nEvidence gate warning: {count} verification claim(s) lack an allowed validation level or concrete evidence source."
    )
}

#[cfg(test)]
#[path = "verification_gate_tests.rs"]
mod tests;
