//! Lifecycle ordering for one requested sub-agent tool call.
//!
//! [execute] prepares input, executes or reports rejection, publishes the raw
//! result, and renders it before assembling [Completed]. [Host] binds the
//! current agent's state, telemetry, and execution services.
mod host;
use super::{Completed, raw::RawResult};
use crate::agent_loop::ToolCall;
pub use host::Host;

/// Execute and publish one sub-agent tool call.
/// # Arguments
/// * `host` — Current agent state and per-call operations.
/// * `call` — Requested call and its original wire arguments.
/// # Returns
/// A rendered completion retaining images and both timeout classifications.
/// Input rejection becomes a failed raw result and follows normal publication.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_swarm_core::agent_loop::tools::call::Host, call: codetether_swarm_core::agent_loop::ToolCall) {
/// let _completed = codetether_swarm_core::agent_loop::tools::call::execute(host, call).await;
/// # }
/// ```
pub async fn execute(host: &mut impl Host, call: ToolCall) -> Completed {
    host.start(&call);
    let raw = match host.prepare(&call).await {
        Ok(args) => host.raw(&call.name, args).await,
        Err(output) => RawResult::failure(output, false),
    };
    host.finished(&call, &raw);
    host.publish(&call, &raw);
    let (output, processing_timeout) = host.render(&call.name, &raw).await;
    Completed {
        id: call.id,
        name: call.name,
        output,
        images: raw.images,
        timed_out: raw.timed_out || processing_timeout,
    }
}
