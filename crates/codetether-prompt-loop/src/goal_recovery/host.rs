use crate::{LoopState, StepFlow};
use codetether_goal_core::GoalStatus;
use codetether_session_events::SessionEvent;
use std::future::Future;
use tokio::sync::mpsc;
/// Native goal storage and continuation services used by recovery.
/// # Implementors
/// The prompt runner supplies its existing goal runtime and live event sender.
pub trait Host: Send {
    /// Native snapshot type carried by session events.
    type Snapshot: Send;
    /// Mutable counters retained across goal turns.
    fn progress(&mut self) -> &mut LoopState;
    /// Whether the stored goal remains active; lookup failures return false.
    fn active(&mut self) -> impl Future<Output = bool> + Send;
    /// Optional sender for retry notifications.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
    /// Persist the stopped status selected by the recovery policy.
    /// # Errors
    /// Returns native storage failures for the caller to log.
    fn set_status(
        &mut self,
        status: GoalStatus,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send;
    /// Ask the native runtime whether another goal turn should run.
    /// # Errors
    /// Returns continuation failures, which recovery treats as a stop.
    fn continue_goal(&mut self) -> impl Future<Output = anyhow::Result<StepFlow>> + Send;
}
