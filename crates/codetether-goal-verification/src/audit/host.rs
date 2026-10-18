//! Native journal and goal-transition operations.
use crate::VerdictRecord;
use anyhow::Result;
use codetether_goal_core::{Goal, GoalStatus};
use std::future::Future;
/// Persistent operations used by audited verification.
/// # Implementors
/// Session runtimes retain their journal paths and revision-checked updates.
pub trait Host {
    /// Read all verdict records for the requested session.
    /// # Errors
    /// Returns journal-resolution or read errors.
    fn read(session: &str) -> impl Future<Output = Result<Vec<VerdictRecord>>> + Send;
    /// Append one verdict record to the session journal.
    /// # Errors
    /// Returns journal-resolution or write errors.
    fn append(session: &str, record: &VerdictRecord) -> impl Future<Output = Result<()>> + Send;
    /// Change status only while the inspected goal revision is still current.
    /// # Errors
    /// Returns persistence or runtime errors without changing newer goals.
    fn set_status_if_current(
        session: &str,
        goal: &Goal,
        status: GoalStatus,
    ) -> impl Future<Output = Result<bool>> + Send;
}
