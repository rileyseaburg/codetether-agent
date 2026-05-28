use super::{HeartbeatState, WorkerStatus};

impl HeartbeatState {
    pub async fn set_status(&self, status: WorkerStatus) {
        *self.status.lock().await = status;
    }

    pub async fn set_task_count(&self, count: usize) {
        *self.active_task_count.lock().await = count;
    }

    pub async fn register_sub_agent(&self, agent_name: String) {
        self.sub_agents.lock().await.insert(agent_name);
    }

    pub async fn deregister_sub_agent(&self, agent_name: &str) {
        self.sub_agents.lock().await.remove(agent_name);
    }

    pub async fn sub_agents_snapshot(&self) -> Vec<String> {
        let mut names = self
            .sub_agents
            .lock()
            .await
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        names.sort();
        names
    }
}
