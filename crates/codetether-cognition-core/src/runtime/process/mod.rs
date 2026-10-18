//! Per-work-item cognition phase dispatch and state updates.
//!
//! [process_work] collects context, generates a thought and dispatches its
//! phase. [Host] supplies model invocation, belief extraction and tool execution;
//! shared phase modules own accounting, proposals, snapshots and state changes.
mod check;
mod compress;
mod propose;
mod reflect;
mod run;
mod test;
use crate::{ThoughtEvent, ThoughtResult, ThoughtWorkItem, beliefs::Belief, runtime::LoopContext};
pub use run::process_work;
use std::{future::Future, sync::Arc};
/// Native inference and tool operations used by phase processing.
/// # Implementors
/// Applications bind thinker clients and their tool registry.
pub trait Host<T, R>: Sync {
    /// Generate one thought, preserving native deterministic fallback behavior.
    fn generate(
        &self,
        thinker: Option<&T>,
        work: &ThoughtWorkItem,
        context: &[ThoughtEvent],
    ) -> impl Future<Output = ThoughtResult> + Send;
    /// Extract structured beliefs from a model-backed thought.
    fn extract(
        &self,
        thinker: Option<&T>,
        persona: &str,
        text: &str,
    ) -> impl Future<Output = Vec<Belief>> + Send;
    /// Run thought-requested tools under the existing persona policy.
    fn run_tools(
        &self,
        ctx: &LoopContext<T, R>,
        tools: &Arc<R>,
        work: &ThoughtWorkItem,
        text: &str,
    ) -> impl Future<Output = Vec<ThoughtEvent>> + Send;
}
