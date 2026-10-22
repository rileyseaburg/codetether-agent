//! Native goal state and revision-checked model edits.
use anyhow::Result;
use codetether_goal_core::{GoalEdit, TaskLog, TaskState, control::GoalControlError};
use serde_json::Value;
use std::future::Future;
/// Storage operations used by model-facing goal tools.
/// # Implementors
/// Native session runtimes retain journal paths, cached state and controller
/// authorization/accounting for model-originated edits.
pub trait Host {
    /// Resolve a canonical user-message reference for a proposed objective.
    ///
    /// Defaults to no evidence. Implementations must read trusted session state,
    /// not accept source metadata from model-provided tool arguments.
    /// # Arguments
    /// * `_session` — Owning durable session identity.
    /// * `_objective` — Proposed goal wording to compare with the source.
    /// # Returns
    /// A canonical source candidate, or `None` when no source can be established.
    /// # Errors
    /// Returns storage or source-decoding errors.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_goal_tools::Host>() -> anyhow::Result<()> {
    /// let _source = H::source("session-123", "Inspect the CLI").await?;
    /// # Ok(()) }
    /// ```
    fn source(
        _session: &str,
        _objective: &str,
    ) -> impl Future<Output = Result<Option<crate::GoalSource>>> + Send {
        async { Ok(None) }
    }
    /// Read the current state and its append-only journal.
    /// # Errors
    /// Returns session-path, journal or runtime-state errors.
    fn current(session: &str) -> impl Future<Output = Result<(TaskLog, TaskState)>> + Send;
    /// Apply a revision-bound model edit using the existing controller.
    /// # Errors
    /// Returns validation, conflict or persistence errors.
    fn update(
        session: &str,
        edit: GoalEdit,
    ) -> impl Future<Output = Result<Value, GoalControlError>> + Send;
}
