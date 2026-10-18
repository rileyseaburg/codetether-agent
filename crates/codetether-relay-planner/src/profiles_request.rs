//! Completion request for profiles planning.
pub(super) fn build(
    model_name: String,
    task: &str,
    requested_agents: usize,
) -> codetether_provider_core::CompletionRequest {
    codetether_provider_core::CompletionRequest {
        model: model_name,
        messages: vec![
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::System,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: "You are a relay-team architect. Return ONLY valid JSON.".to_string(),
                }],
            },
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::User,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nDesign a task-specific relay team.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"profiles\": [\n    {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n  ]\n}}\n\
                         Requirements:\n\
                         - Return {} profiles\n\
                         - Names must be short kebab-case\n\
                         - Capabilities must be concise skill tags\n\
                         - Missions should be concrete and handoff-friendly",
                        requested_agents
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(1200),
        stop: Vec::new(),
    }
}
