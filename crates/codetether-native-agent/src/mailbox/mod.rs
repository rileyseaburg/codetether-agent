//! Native durable child-mailbox persistence with application turn execution.
//!
//! [Mailbox] binds the shared mailbox queue to canonical agent sidecar paths.
//! [Execution] retains the application's existing queued-turn execution and
//! parameter conversion while this module owns load/save/remove operations.
use codetether_child_mailbox::Item;
use codetether_collaboration_state::execution::AgentRunGuard;
use codetether_tool_core::ToolResult;
use std::marker::PhantomData;
mod disk;
mod host;
/// Application-owned queued turn execution.
/// # Implementors
/// Preserve the existing item-to-input conversion and active run guard.
#[async_trait::async_trait]
pub trait Execution: Send + Sync + 'static {
    /// Run the queued item using the already-acquired agent execution guard.
    /// # Errors
    /// Returns the native child turn's execution failure.
    async fn execute(
        agent_id: String,
        item: Item,
        guard: AgentRunGuard,
    ) -> anyhow::Result<ToolResult>;
}
/// Native durable mailbox host with execution callback E.
/// # Examples
/// ```
/// let _: Option<codetether_native_agent::Mailbox<()>> = None;
/// ```
pub struct Mailbox<E>(PhantomData<fn() -> E>);

mod params;
pub use params::queued_mailbox_params;
mod binding;
