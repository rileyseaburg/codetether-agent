use super::Native;
use crate::a2a::worker::{TaskContext, task_dispatch_special};
use codetether_a2a_worker_core::{outcome::TaskInner, task_driver::claimed::Special};
impl Special for Native<'_> {
    async fn special(
        &self,
        task: &serde_json::Value,
        title: &str,
        context: &TaskContext,
    ) -> Option<TaskInner> {
        task_dispatch_special::dispatch_special_task(self.0, task, title, context).await
    }
}
