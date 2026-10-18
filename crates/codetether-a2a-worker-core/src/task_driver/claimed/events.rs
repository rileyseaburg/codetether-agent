use crate::task_timeline::TaskTimeline;
use std::{future::Future, sync::Arc};
/// Shared callback for streamed task text.
pub type Output = Arc<dyn Fn(String) + Send + Sync + 'static>;
/// Native progress synchronization and task event sinks.
/// # Implementors
/// Worker adapters retain transport and process sink ownership.
pub trait Events: Sync {
    /// Publish the latest timeline state.
    fn sync(&self, timeline: &TaskTimeline) -> impl Future<Output = ()> + Send;
    /// Build the task's text-output callback.
    fn output(&self, task_id: &str) -> Output;
    /// Install structured event delivery for this task.
    fn install(&self, task_id: &str);
    /// Clear structured event delivery after execution returns.
    fn clear(&self);
}
