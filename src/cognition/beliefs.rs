//! Shared belief contracts and the application thinker adapter.
use super::thinker::ThinkerClient;
pub use codetether_cognition_core::beliefs::{Belief, BeliefStatus};

/// Extract beliefs from a thought using structured LLM extraction.
///
/// Returns a Vec of new Belief structs. The caller handles duplicate detection
/// and insertion into the belief store.
pub async fn extract_beliefs_from_thought(
    thinker: Option<&ThinkerClient>,
    persona_id: &str,
    thought_text: &str,
) -> Vec<Belief> {
    let Some(client) = thinker else {
        return Vec::new();
    };

    let (system_prompt, user_prompt) =
        codetether_cognition_core::beliefs::extraction::extraction_prompts(thought_text);

    let output = match client.think(&system_prompt, &user_prompt).await {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs(
        persona_id,
        &output.text,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn extract_beliefs_no_thinker_returns_empty() {
        let result = extract_beliefs_from_thought(None, "p1", "some thought").await;
        assert!(result.is_empty());
    }
}
