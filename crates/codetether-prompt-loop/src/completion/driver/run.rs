use super::Host;
use crate::completion::Attempt;
use codetether_provider_core::CompletionResponse;
/// Execute provider attempts until success or an unrecoverable failure.
/// # Arguments
/// `host` supplies native services; `step` identifies the prompt-loop step.
/// # Returns
/// The first successful completion after accounting and counter reset.
/// # Errors
/// Propagates initialization, checkpoint, recovery and unrecoverable request errors.
/// # Examples
/// ```no_run
/// # async fn run(host: &mut impl codetether_prompt_loop::completion::driver::Host) -> anyhow::Result<()> {
/// let _response = codetether_prompt_loop::completion::driver::complete(host, 1).await?;
/// # Ok(()) }
/// ```
pub async fn complete(host: &mut impl Host, step: usize) -> anyhow::Result<CompletionResponse> {
    let mut attempt = host.initialize(step).await?;
    let started = std::time::Instant::now();
    loop {
        attempt.count += 1;
        let result = send(host, &mut attempt).await;
        match result {
            Ok(response) => {
                host.succeeded(attempt.bucket);
                host.usage(&response, started.elapsed()).await;
                host.progress().goal_failure_signature = None;
                host.progress().goal_failure_repeats = 0;
                return Ok(response);
            }
            Err(error) => {
                host.checkpoint(&error).await?;
                if !host.recover(step, &mut attempt, &error).await? {
                    return Err(error);
                }
            }
        }
    }
}
async fn send(host: &mut impl Host, attempt: &mut Attempt) -> anyhow::Result<CompletionResponse> {
    let request = host.build(attempt).await?;
    host.dispatch(request).await
}
