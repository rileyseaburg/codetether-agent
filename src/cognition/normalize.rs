//! Normalization of raw model output into stored thought text.

use super::normalize_shape::{labeled_section, looks_meta};
use super::{ThoughtEvent, ThoughtWorkItem, phase_default, placeholder, text_util};

/// Clean `raw` into stored thought text, substituting deterministic text when
/// the model produced placeholders or meta narration.
pub(super) fn normalize_thought_output(
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
    raw: &str,
) -> String {
    let trimmed = text_util::trim_for_storage(raw, 2_000);
    if trimmed.trim().is_empty() {
        return phase_default::phase_default_text(work, context);
    }
    if let Some(labeled) = labeled_section(&trimmed) {
        return labeled;
    }
    if looks_meta(&trimmed) || placeholder::has_template_placeholder_values(&trimmed) {
        return phase_default::phase_default_text(work, context);
    }
    trimmed
}
