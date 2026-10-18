//! Completion request for okr planning.
pub(super) fn build(
    model_name: String,
    task: &str,
    agent_count: usize,
) -> codetether_provider_core::CompletionRequest {
    codetether_provider_core::CompletionRequest {
        model: model_name,
        messages: vec![
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::System,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: "You write OKRs for execution governance. Return ONLY valid JSON."
                        .to_string(),
                }],
            },
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::User,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nTeam size: {agent_count}\n\n\
                         Propose ONE objective and 3-7 measurable key results for executing this task via an AI relay.\n\
                         Key results must be quantitative (numeric target_value + unit).\n\n\
                         Return JSON ONLY (no markdown):\n\
                         {{\n  \"title\": \"...\",\n  \"description\": \"...\",\n  \"key_results\": [\n    {{\"title\":\"...\",\"target_value\":123,\"unit\":\"%|count|tests|files|items\"}}\n  ]\n}}\n\n\
                         Rules:\n\
                         - Avoid vague KRs like 'do better'\n\
                         - Prefer engineering outcomes (tests passing, endpoints implemented, docs updated, errors=0)\n\
                         - If unsure about a unit, use 'count'"
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(0.4),
        top_p: Some(0.9),
        max_tokens: Some(900),
        stop: Vec::new(),
    }
}
