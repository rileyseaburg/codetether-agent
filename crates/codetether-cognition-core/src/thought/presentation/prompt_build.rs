//! Prompt construction for one thought phase.

use super::{ThoughtEvent, ThoughtWorkItem, prompt_phase, text_util};

/// Shared instruction preamble for every phase.
const SYSTEM_PROMPT: &str = concat!(
    "You are the internal cognition engine for a persistent autonomous persona. ",
    "Respond with concise plain text only. Do not include markdown, XML, or code fences. ",
    "Write as an operational process update, not meta narration. ",
    "Do not say phrases like 'I need to', 'we need to', 'I will', or describe your own ",
    "reasoning process. Output concrete findings, checks, risks, and next actions. ",
    "Fill every labeled field with concrete content. Never output placeholders such as ",
    "'...', '<...>', 'TBD', or 'TO",
    "DO'."
);

/// Build the `(system, user)` prompt pair for `work`.
///
/// # Arguments
/// * `work` - Persona and phase whose text is being generated.
/// * `context` - Recent events in chronological order.
///
/// # Returns
/// System prompt followed by the populated user prompt.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{ThoughtWorkItem, ThoughtPhase, ThoughtEvent, ThoughtEventType};
/// use codetether_cognition_core::thought::presentation::build_phase_prompts;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Example".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Observe };
/// let (_, user) = build_phase_prompts(&work, &[]);
/// assert!(user.contains("persona_id: p1"));
/// ```
pub fn build_phase_prompts(work: &ThoughtWorkItem, context: &[ThoughtEvent]) -> (String, String) {
    let context_lines = if context.is_empty() {
        "none".to_string()
    } else {
        context
            .iter()
            .map(format_context_event)
            .collect::<Vec<_>>()
            .join("\n")
    };

    let user_prompt = format!(
        "phase: {phase}\npersona_id: {persona_id}\npersona_name: {persona_name}\nrole: {role}\ncharter: {charter}\nthought_count: {count}\nrecent_context:\n{context_lines}\n\ninstruction:\n{instruction}",
        phase = work.phase.as_str(),
        persona_id = work.persona_id,
        persona_name = work.persona_name,
        role = work.role,
        charter = work.charter,
        count = work.thought_count,
        instruction = prompt_phase::instruction(work.phase),
    );
    (SYSTEM_PROMPT.to_string(), user_prompt)
}

/// Render one prior event as a single compact context line.
///
/// # Arguments
/// * `event` - Prior event to format.
///
/// # Returns
/// One compact line containing event label, time and clipped payload.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{ThoughtWorkItem, ThoughtPhase, ThoughtEvent, ThoughtEventType};
/// use codetether_cognition_core::thought::presentation::format_context_event;
/// let event = ThoughtEvent { id: "e1".into(), event_type: ThoughtEventType::CheckResult, persona_id: None, swarm_id: None, timestamp: chrono::Utc::now(), payload: serde_json::json!({}) };
/// assert!(format_context_event(&event).starts_with("check_result "));
/// ```
pub fn format_context_event(event: &ThoughtEvent) -> String {
    let payload = serde_json::to_string(&event.payload).unwrap_or_else(|_| "{}".to_string());
    format!(
        "{} {} {}",
        event.event_type.as_str(),
        event.timestamp.to_rfc3339(),
        text_util::trim_for_storage(&payload, 220)
    )
}
