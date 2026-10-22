//! Batch mode selection and ordered sequential execution.
use crate::*;
/// Execute and record a batch using native policy and dispatch.
/// # Arguments
/// Mutable session and ordered model-emitted tool calls.
/// # Returns
/// Returns after all selected execution results are recorded.
/// # Errors
/// Stops before an edit if persistence fails, or after recording its result if refresh fails.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_replay::*;
/// # async fn example<H: Host>(host: &H, session: &mut H::Session) -> anyhow::Result<()> {
/// execute(host,session,vec![]).await?;
/// # Ok(())
/// # }
/// ```
pub async fn execute<H: Host>(
    host: &H,
    session: &mut H::Session,
    tool_calls: Vec<ContentPart>,
) -> anyhow::Result<()> {
    if can_parallelize::<H>(session, &tool_calls) {
        let results = crate::parallel::execute(host, session, &tool_calls).await;
        record_results::<H>(session, tool_calls, results);
    } else {
        super::sequential::execute(host, session, tool_calls).await?;
    }
    Ok(())
}
