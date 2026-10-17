//! Detection of unfilled template placeholders in model output.

/// Labeled fields the model must fill with concrete content.
const LABELS: &[&str] = &[
    "goal",
    "signals",
    "uncertainty",
    "next_action",
    "hypothesis",
    "rationale",
    "business_risk",
    "validation_next_action",
    "check",
    "procedure",
    "expected_result",
    "evidence_quality",
    "escalation_trigger",
    "state_summary",
    "retained_facts",
    "open_risks",
    "next_process_step",
];

/// Whether `text` still contains an unfilled placeholder value.
///
/// # Arguments
/// * `text` - Text to inspect for labels or placeholders.
///
/// # Returns
/// Whether a known unfilled template pattern is present.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{ThoughtWorkItem, ThoughtPhase, ThoughtEvent, ThoughtEventType};
/// use codetether_cognition_core::thought::presentation::has_template_placeholder_values;
/// assert!(has_template_placeholder_values("Goal: ..."));
/// ```
pub fn has_template_placeholder_values(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    LABELS
        .iter()
        .any(|label| lower.contains(&format!("{label}: ...")))
        || lower.contains("<...")
        || lower.contains("tbd")
        || lower.contains(concat!("to", "do"))
}

/// Locate the first process label so preamble text can be stripped.
///
/// # Arguments
/// * `text` - Text to inspect for labels or placeholders.
///
/// # Returns
/// Byte offset of the earliest recognized label, or None.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{ThoughtWorkItem, ThoughtPhase, ThoughtEvent, ThoughtEventType};
/// use codetether_cognition_core::thought::presentation::find_process_label_start;
/// assert_eq!(find_process_label_start("Phase: Observe"), Some(0));
/// ```
pub fn find_process_label_start(text: &str) -> Option<usize> {
    [
        "Phase: Observe",
        "Phase: Reflect",
        "Phase: Test",
        "Phase: Compress",
        "Phase:",
    ]
    .iter()
    .filter_map(|label| text.find(label))
    .min()
}
