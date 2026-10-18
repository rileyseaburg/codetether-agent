//! Completion request for spawn planning.
pub(super) fn build(
    model_name: String,
    task: &str,
    team: &str,
    output_excerpt: &str,
    round: usize,
) -> codetether_provider_core::CompletionRequest {
    codetether_provider_core::CompletionRequest {
        model: model_name,
        messages: vec![
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::System,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: "You are a relay scaling controller. Return ONLY valid JSON.".to_string(),
                }],
            },
            codetether_provider_core::Message {
                role: codetether_provider_core::Role::User,
                content: vec![codetether_provider_core::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nRound: {round}\nCurrent team: {team}\n\
                         Latest handoff excerpt:\n{output_excerpt}\n\n\
                         Decide whether the team needs one additional specialist right now.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"spawn\": true|false,\n  \"reason\": \"...\",\n  \"profile\": {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n}}\n\
                         If spawn=false, profile may be null or omitted."
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(420),
        stop: Vec::new(),
    }
}
