//! Structured belief extraction using a configured shared thinker client.
use super::Belief;
use crate::thinker::client::{Client, Host};

/// Extract beliefs from a thought using structured LLM extraction.
///
/// Returns a Vec of new Belief structs. The caller handles duplicate detection
/// and insertion into the belief store.
pub async fn extract_beliefs_from_thought<H: Host>(
    thinker: Option<&Client<H>>,
    persona_id: &str,
    thought_text: &str,
) -> Vec<Belief> {
    let Some(client) = thinker else {
        return Vec::new();
    };

    let (system_prompt, user_prompt) = super::extraction::extraction_prompts(thought_text);

    let output = match client.think(&system_prompt, &user_prompt).await {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    super::extraction::parse_extracted_beliefs(persona_id, &output.text)
}
