//! Persistent subprocess isolation for native desktop operations.
//!
//! `execute` serializes requests; `serve` is the early-dispatched worker entry.
//! Failed requests are never retried because native input may already have run.

#[path = "worker/client.rs"]
mod client;
#[path = "worker/codec.rs"]
mod codec;
#[path = "worker/diagnostics.rs"]
mod diagnostics;
#[path = "worker/failure.rs"]
mod failure;
#[path = "worker/framing.rs"]
mod framing;
#[path = "worker/process.rs"]
mod process;
#[path = "worker/queue.rs"]
mod queue;
#[path = "worker/server.rs"]
mod server;
#[cfg(test)]
#[path = "worker/tests.rs"]
mod tests;
#[path = "worker/transport.rs"]
mod transport;

use super::input::ComputerUseInput;
use crate::tool::ToolResult;

/// Execute one desktop request in the persistent isolated worker.
///
/// # Arguments
/// * `input` — The desktop action to perform exactly once.
/// # Returns
/// The unchanged worker result, or a structured worker failure result.
/// # Errors
/// Infrastructure failures are returned as unsuccessful `ToolResult` values.
/// # Examples
/// ```rust,no_run
/// # async fn example(input: codetether_agent::tool::computer_use::input::ComputerUseInput) -> anyhow::Result<()> {
/// let result = codetether_agent::tool::computer_use::worker::execute(input).await?;
/// assert!(result.success);
/// # Ok(()) }
/// ```
pub async fn execute(input: ComputerUseInput) -> anyhow::Result<ToolResult> {
    Ok(client::execute(input).await)
}

/// Serve newline-delimited desktop requests without initializing agent services.
///
/// # Arguments
/// Reads only standard input and writes protocol responses to standard output.
/// # Returns
/// Returns when the controlling parent closes standard input.
/// # Errors
/// Returns an error on malformed, oversized, or broken protocol streams.
/// # Examples
/// ```text
/// codetether windows computer-use-worker
/// ```
pub async fn serve() -> anyhow::Result<()> {
    server::serve().await
}
