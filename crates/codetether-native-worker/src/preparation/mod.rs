//! Native workspace resolution and shared task-session preparation.
//!
//! [`prepare`] uses the application's [`Loader`] and its [`SessionState`]
//! implementation to preserve session selection, workspace resolution,
//! provenance assignment and repository preparation order. Storage remains
//! supplied by the application; HTTP and worker state belong to this crate.
use crate::WorkerTaskRuntime;
pub use codetether_a2a_worker_core::preparation::session::{Loader, SessionState};
use codetether_a2a_worker_core::{task_config::TaskContext, task_timeline::TaskTimeline};
use codetether_provenance::ClaimProvenance;
use std::marker::PhantomData;
mod host;

/// Load and prepare a claimed task's session using native worker services.
///
/// # Arguments
/// * `runtime` - Worker HTTP services and identity.
/// * `task_id` - Identifier used for preparation and progress checkpoints.
/// * `context` - Task configuration including session and workspace selection.
/// * `timeline` - Mutable progress timeline for the task.
/// * `claim_provenance` - Attribution accepted during claiming.
/// * `provider_keys` - Optional provider credentials accepted with the claim.
///
/// # Returns
/// The prepared application session and normalized agent type.
///
/// # Errors
/// Propagates session loading, workspace resolution and repository setup errors.
///
/// # Examples
/// ```no_run
/// # async fn example<L: codetether_native_worker::preparation::Loader>(runtime: &codetether_native_worker::WorkerTaskRuntime, context: &codetether_a2a_worker_core::task_config::TaskContext, timeline: &mut codetether_a2a_worker_core::task_timeline::TaskTimeline, claim: &codetether_provenance::ClaimProvenance) -> anyhow::Result<()> {
/// let _prepared = codetether_native_worker::preparation::prepare::<L>(runtime, "task", context, timeline, claim, None).await?;
/// # Ok(()) }
/// ```
pub async fn prepare<L: Loader>(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    context: &TaskContext,
    timeline: &mut TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> anyhow::Result<(L::Session, String)> {
    codetether_a2a_worker_core::preparation::session::prepare(
        &host::Native::<L> {
            runtime,
            loader: PhantomData,
        },
        task_id,
        context,
        timeline,
        claim_provenance,
        provider_keys,
    )
    .await
}
