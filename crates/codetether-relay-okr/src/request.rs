//! Provider completion parameters for relay OKR drafting.
pub(crate) fn build(
    task: &str,
    model_name: String,
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
                    text: crate::prompt::user(task, agent_count),
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
