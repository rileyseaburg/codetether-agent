//! Child completion projection into mux task state.

use std::sync::Arc;

use super::entry::AgentTask;

pub(super) fn start(
    mut child: tokio::process::Child,
    task: Arc<AgentTask>,
    readers: Vec<tokio::task::JoinHandle<()>>,
) {
    tokio::spawn(async move {
        let exit_code = child
            .wait()
            .await
            .ok()
            .and_then(|status| status.code())
            .unwrap_or(1);
        for reader in readers {
            let _ = reader.await;
        }
        task.finish(exit_code);
    });
}
