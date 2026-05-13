//! Prompt packing for semantic synthesis.

use super::evidence::Evidence;

/// Build the final model input from query and evidence.
pub fn user_prompt(query: &str, evidence: &Evidence) -> String {
    format!(
        "Question:\n{query}\n\nEvidence records selected: {}\n\nTyped evidence pack:\n```\n{}\n```\n\nAnswer using only the evidence.",
        evidence.records.len(),
        evidence.text
    )
}

/// System instruction for the evidence-first synthesis step.
pub fn system_prompt() -> String {
    "You are an evidence-first code analysis engine. \
     Use only supplied typed evidence records, cite concrete names and paths when present, \
     and say when the evidence is insufficient."
        .into()
}
