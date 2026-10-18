use super::{Host, ToolCall};
/// Execute calls in planner order, falling back when a parallel group declines.
/// # Arguments
/// `host` executes calls; `step` identifies the prompt step; `calls` contains
/// the ordered provider-issued calls.
/// # Returns
/// Success after all calls or an execution-requested early stop.
/// # Errors
/// Propagates sequential execution failures.
/// # Examples
/// ```no_run
/// # async fn dispatch(host: &mut impl codetether_tool_parallel::dispatch::Host) -> anyhow::Result<()> {
/// codetether_tool_parallel::dispatch::run(host, 1, Vec::new()).await?;
/// # Ok(()) }
/// ```
pub async fn run(host: &mut impl Host, step: usize, calls: Vec<ToolCall>) -> anyhow::Result<()> {
    let plan = crate::plan(&calls, host.allow_parallel());
    for batch in plan {
        let stop = match batch {
            crate::Batch::Parallel(range) => {
                if host.parallel(&calls[range.clone()]).await {
                    false
                } else {
                    super::sequential::run(host, step, &calls, range).await?
                }
            }
            crate::Batch::Single(index) => {
                super::sequential::run(host, step, &calls, index..index + 1).await?
            }
        };
        if stop {
            break;
        }
    }
    Ok(())
}
