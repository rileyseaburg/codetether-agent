//! Parent lifecycle; a failed or cancelled exchange drops its child without retry.
use super::{codec, failure::Failure, framing::REQUEST_LIMIT, process::Process, queue};
use crate::tool::{ToolResult, computer_use::input::ComputerUseInput};
use std::time::Duration;
use tokio::time::timeout;

const RESPONSE_TIMEOUT: Duration = Duration::from_secs(120);
const QUEUE_TIMEOUT: Duration = Duration::from_secs(125);

#[tracing::instrument(name = "computer_use_worker", skip_all, fields(action = ?input.action))]
pub(super) async fn execute(input: ComputerUseInput) -> ToolResult {
    let request = match codec::encode(&input, REQUEST_LIMIT) {
        Ok(request) => request,
        Err(_) => return Failure::RequestTooLarge.result(),
    };
    let mut slot = match queue::acquire(QUEUE_TIMEOUT).await {
        Ok(slot) => slot,
        Err(error) => return error.result(),
    };
    run(&mut slot, &request, RESPONSE_TIMEOUT, Process::spawn).await
}

pub(super) async fn run(
    slot: &mut Option<Process>,
    request: &[u8],
    deadline: Duration,
    spawn: impl FnOnce() -> Result<Process, Failure>,
) -> ToolResult {
    // Keep ownership outside the shared slot across the await: cancellation drops
    // the child and its response pipe instead of leaving a late reply for reuse.
    let mut worker = match slot.take().map(Ok).unwrap_or_else(spawn) {
        Ok(worker) => worker,
        Err(error) => return error.result(),
    };
    let result = timeout(deadline, worker.exchange(request)).await;
    match result {
        Ok(Ok(result)) => {
            *slot = Some(worker);
            result
        }
        Ok(Err(error)) => worker.failed(error),
        Err(_) => worker.failed(Failure::Timeout),
    }
}
