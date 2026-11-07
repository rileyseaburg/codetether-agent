//! Bounded backend traversal; only accepted results terminate fallback.
use crate::{Backend, attempt::Attempt, report};
use codetether_tool_core::ToolResult;
pub(crate) async fn search<F, Fut>(query: &str, order: Vec<Backend>, mut attempt: F) -> ToolResult
where
    F: FnMut(Backend) -> Fut,
    Fut: std::future::Future<Output = Attempt>,
{
    let mut attempts = Vec::with_capacity(order.len());
    for backend in order {
        let result = attempt(backend).await;
        let done = result.outcome.is_ok();
        attempts.push(result);
        if done {
            break;
        }
    }
    report::render(query, &attempts)
}
