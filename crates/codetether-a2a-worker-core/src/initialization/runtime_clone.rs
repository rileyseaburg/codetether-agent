use super::WorkerTaskRuntime;
impl<C: Clone, B> Clone for WorkerTaskRuntime<C, B> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            server: self.server.clone(),
            token: self.token.clone(),
            worker_id: self.worker_id.clone(),
            agent_name: self.agent_name.clone(),
            processing: self.processing.clone(),
            max_concurrent_tasks: self.max_concurrent_tasks,
            auto_approve: self.auto_approve,
            bus: self.bus.clone(),
            task_progress: self.task_progress.clone(),
            workspace_ids: self.workspace_ids.clone(),
        }
    }
}
