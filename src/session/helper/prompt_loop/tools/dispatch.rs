//! Execute an ordered tool plan with read-only parallel groups.

use std::ops::Range;

use anyhow::Result;

use super::super::Runner;
use super::ToolCall;

pub(super) async fn run(runner: &mut Runner<'_>, step: usize, calls: Vec<ToolCall>) -> Result<()> {
    let plan = super::super::super::tool_parallel::plan(&calls, runner.events.is_some());
    for batch in plan {
        let stop = match batch {
            super::super::super::tool_parallel::Batch::Parallel(range) => {
                if super::parallel::run(runner, &calls[range.clone()]).await {
                    false
                } else {
                    sequential(runner, step, &calls, range).await?
                }
            }
            super::super::super::tool_parallel::Batch::Single(index) => {
                sequential(runner, step, &calls, index..index + 1).await?
            }
        };
        if stop {
            break;
        }
    }
    Ok(())
}

async fn sequential(
    runner: &mut Runner<'_>,
    step: usize,
    calls: &[ToolCall],
    range: Range<usize>,
) -> Result<bool> {
    for (id, name, input) in &calls[range] {
        let call = super::call::Call::new(id.clone(), name.clone(), input.clone());
        if super::call::run(runner, step, call).await? {
            return Ok(true);
        }
    }
    Ok(false)
}
