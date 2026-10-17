//! Normalization of raw model output into stored thought text.

use super::normalize_shape::{labeled_section, looks_meta};
use super::{ThoughtEvent, ThoughtWorkItem, phase_default, placeholder, text_util};

/// Clean `raw` into stored thought text, substituting deterministic text when
/// the model produced placeholders or meta narration.
///
/// # Arguments
/// * `work` - Persona and phase whose text is being generated.
/// * `context` - Recent events in chronological order.
/// * `raw` - Raw model output.
///
/// # Returns
/// Stored thought text, using a deterministic fallback when needed.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{ThoughtWorkItem, ThoughtPhase, ThoughtEvent, ThoughtEventType};
/// use codetether_cognition_core::thought::presentation::normalize_thought_output;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Example".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Observe };
/// assert_eq!(normalize_thought_output(&work, &[], "Clear result."), "Clear result.");
/// ```
pub fn normalize_thought_output(
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
