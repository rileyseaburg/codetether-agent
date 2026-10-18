//! Preserve final-answer handling and bounded validation retries.
use super::Host;
use crate::StepFlow;
use codetether_prompt_policy::loop_constants as limits;
use codetether_provider_core::CompletionResponse;
/// Insert an answer and decide whether validation or steering needs another step.
/// # Arguments
/// * `host` — Native session and validation bindings.
/// * `response` — Assistant completion to append.
/// # Returns
/// The next prompt-loop action.
/// # Errors
/// Propagates validation, retry exhaustion and final goal-flow failures.
/// # Examples
/// ```no_run
/// # async fn finish<H: codetether_prompt_loop::terminal::Host>(host: &mut H, response: codetether_provider_core::CompletionResponse) -> anyhow::Result<()> {
/// let _flow = codetether_prompt_loop::terminal::finish_or_retry(host, response).await?;
/// # Ok(()) }
/// ```
pub async fn finish_or_retry(
    host: &mut impl Host,
    response: CompletionResponse,
) -> anyhow::Result<StepFlow> {
    host.append(response.message);
    if host.drain_steering() > 0 {
        return Ok(StepFlow::Continue);
    }
    if !codetether_build_policy::is_build_agent(host.agent()) {
        return host.final_flow().await;
    }
    let report = host.validate().await?;
    let Some(report) = report else {
        return host.final_flow().await;
    };
    host.progress().validation_retries += 1;
    if host.progress().validation_retries >= limits::POST_EDIT_VALIDATION_MAX_RETRIES {
        anyhow::bail!(
            "Post-edit validation failed after {} attempts.\n\n{}",
            limits::POST_EDIT_VALIDATION_MAX_RETRIES,
            report
        );
    }
    host.append(crate::response::corrective_message(&report));
    host.progress().output.clear();
    Ok(StepFlow::Continue)
}
