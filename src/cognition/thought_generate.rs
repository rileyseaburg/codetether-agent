//! Thought generation via the thinker, with deterministic recovery.

use tokio::time::Instant;

use super::thought_result::{model_result, recovered};
use super::{ThinkerClient, ThoughtEvent, ThoughtResult, ThoughtWorkItem, normalize, prompt_build};

/// Generate one thought for `work`, using the thinker when it is configured.
///
/// Uses deterministic phase text when no thinker exists, the thinker errors, or
/// it returns empty output.
pub(super) async fn generate_phase_thought(
    thinker: Option<&ThinkerClient>,
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
) -> ThoughtResult {
    let started_at = Instant::now();
    if let Some(client) = thinker {
        let (system_prompt, user_prompt) = prompt_build::build_phase_prompts(work, context);
        match client.think(&system_prompt, &user_prompt).await {
            Ok(output) => {
                let thinking = normalize::normalize_thought_output(work, context, &output.text);
                if !thinking.is_empty() {
                    return model_result(output, thinking, started_at);
                }
            }
            Err(error) => return recovered(work, context, started_at, Some(error.to_string())),
        }
    }
    recovered(work, context, started_at, None)
}
