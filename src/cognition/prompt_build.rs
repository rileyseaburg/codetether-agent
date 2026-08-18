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
pub(super) fn build_phase_prompts(
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
) -> (String, String) {
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
pub(super) fn format_context_event(event: &ThoughtEvent) -> String {
    let payload = serde_json::to_string(&event.payload).unwrap_or_else(|_| "{}".to_string());
    format!(
        "{} {} {}",
        event.event_type.as_str(),
        event.timestamp.to_rfc3339(),
        text_util::trim_for_storage(&payload, 220)
    )
}
