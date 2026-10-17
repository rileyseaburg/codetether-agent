//! Shared durable-child readiness entrypoint for remaining application tools.
use crate::{MessagingHost, RuntimeContext};
use anyhow::Result;
use codetether_tool_core::ToolResult;
/// Ensure a child is open before a collaboration operation.
/// # Arguments
/// * `context` — Trusted parent settings and session identity.
/// * `target` — Child identity resolved by the existing runtime.
/// # Returns
/// A refusal/not-found result, or none when the child is ready.
/// # Errors
/// Propagates residency errors.
/// # Examples
/// Messaging tools call this before selecting a delivery path.
pub async fn ensure_ready<H: MessagingHost>(
    context: &RuntimeContext,
    target: &str,
) -> Result<Option<ToolResult>> {
    crate::ensure::ready::<H>(context, target).await
}
