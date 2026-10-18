//! Native compatibility adapter for shared cognition belief extraction.
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
    codetether_cognition_core::beliefs::extract::extract_beliefs_from_thought(
        thinker,
        persona_id,
        thought_text,
    )
    .await
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
