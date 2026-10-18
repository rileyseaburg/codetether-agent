//! Model-backed thought generation with deterministic phase recovery.
use super::presentation as normalize;
use super::presentation as prompt_build;
use super::result::{model_result, recovered};
use crate::{
    ThoughtEvent, ThoughtResult, ThoughtWorkItem,
    thinker::client::{Client, Host},
};
use tokio::time::Instant;
/// Generate one thought for `work`, using the thinker when it is configured.
///
/// Uses deterministic phase text when no thinker exists, the thinker errors, or
/// it returns empty output.
pub async fn generate_phase_thought<H: Host>(
    thinker: Option<&Client<H>>,
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
