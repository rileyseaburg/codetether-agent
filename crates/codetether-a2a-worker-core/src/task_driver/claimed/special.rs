use crate::{outcome::TaskInner, task_config::TaskContext};
use std::future::Future;
/// Native dispatch for special tasks that bypass normal agent sessions.
/// # Implementors
/// Worker adapters retain clone/forage task execution.
pub trait Special: Sync {
    /// Return a handled special-task result, or None for normal execution.
    fn special(
        &self,
        task: &serde_json::Value,
        title: &str,
        context: &TaskContext,
    ) -> impl Future<Output = Option<TaskInner>> + Send;
}
