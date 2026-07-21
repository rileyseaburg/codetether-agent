//! Event-driven replay reads for one agent task.

use super::entry::AgentTask;

impl AgentTask {
    pub(super) async fn read(&self, offset: u64) -> (Vec<u8>, u64, bool, Option<i32>) {
        let mut changed = self.changed.subscribe();
        let mut chunk = self.output.lock().unwrap().read(offset);
        if chunk.0.is_empty() && self.running() {
            let _ =
                tokio::time::timeout(std::time::Duration::from_secs(5), changed.changed()).await;
            chunk = self.output.lock().unwrap().read(offset);
        }
        (
            chunk.0,
            chunk.1,
            self.running(),
            *self.exit_code.lock().unwrap(),
        )
    }
}
