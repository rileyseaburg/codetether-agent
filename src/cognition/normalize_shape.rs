//! Meta-narration detection in model output.

/// Meta-narration phrases that indicate the model described its own process
/// instead of producing an operational update.
const META: &[&str] = &[
    "we need to",
    "i need to",
    "must output",
    "let's ",
    "we have to",
];

/// Whether the text reads as self-narration rather than an update.
pub(super) fn looks_meta(trimmed: &str) -> bool {
    let lower = trimmed.to_ascii_lowercase();
    lower.starts_with("we need")
        || lower.starts_with("i need")
        || META.iter().any(|needle| lower.contains(needle))
}

/// Prefer process-labeled content when the model emitted a preamble first.
pub(super) fn labeled_section(trimmed: &str) -> Option<String> {
    let idx = super::placeholder::find_process_label_start(trimmed)?;
    let candidate = trimmed[idx..].trim();
    if candidate.is_empty()
        || candidate.contains('<')
        || super::placeholder::has_template_placeholder_values(candidate)
    {
        return None;
    }
    // Collapse multi-line structured output to one pipe-delimited line.
    let collapsed = candidate
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" | ");
    let cleaned = collapsed.trim_matches('"').trim_matches('\'').trim();
    if cleaned.starts_with("Phase:") {
        return Some(cleaned.to_string());
    }
    Some(collapsed)
}
