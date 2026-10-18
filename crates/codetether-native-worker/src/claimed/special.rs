//! Policy-gated special tasks using native worker services.
use super::Native;
use crate::special::{Forage, dispatch_special_task};
use codetether_a2a_worker_core::{
    outcome::TaskInner, task_config::TaskContext, task_driver::claimed::Special,
};
impl<S: Sync, F: Forage> Special for Native<'_, S, F> {
    async fn special(
        &self,
        task: &serde_json::Value,
        title: &str,
        context: &TaskContext,
    ) -> Option<TaskInner> {
        dispatch_special_task::<F>(self.runtime, task, title, context).await
    }
}
