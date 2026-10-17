//! Worker state tasks operations.
use super::*;
impl WorkerServerState {
    /// Set the task notification channel (called from worker)
    ///
    /// # Arguments
    /// tx receives task identifiers pushed to the HTTP endpoint.
    ///
    /// # Returns
    /// Completes after installing the sender.
    ///
    /// # Examples
    /// ```
    /// async fn attach(state: &codetether_worker_http::WorkerServerState) {
    /// let (tx, _rx) = tokio::sync::mpsc::channel(8);
    /// state.set_task_notification_channel(tx).await;
    /// }
    /// ```
    pub async fn set_task_notification_channel(&self, tx: mpsc::Sender<String>) {
        *self.task_notification_tx.lock().await = Some(tx);
    }

    /// Notify worker of a new task (called from /task endpoint)
    ///
    /// # Arguments
    /// task_id is the task identifier sent to the worker.
    ///
    /// # Returns
    /// True on successful delivery; false if no channel is attached or its receiver is closed.
    ///
    /// # Examples
    /// ```
    /// async fn notify(state: &codetether_worker_http::WorkerServerState) {
    /// let delivered = state.notify_new_task("task-1").await;
    /// }
    /// ```
    pub async fn notify_new_task(&self, task_id: &str) -> bool {
        let tx = self.task_notification_tx.lock().await.clone();
        if let Some(tx) = tx {
            match tx.send(task_id.to_string()).await {
                Ok(()) => {
                    tracing::debug!(target: "codetether_agent::worker_server", "Notified worker of new task: {}", task_id);
                    true
                }
                Err(error) => {
                    tracing::warn!(target: "codetether_agent::worker_server",
                        task_id,
                        error = %error,
                        "Worker task notification channel is closed"
                    );
                    false
                }
            }
        } else {
            tracing::warn!(target: "codetether_agent::worker_server",
                task_id,
                "Worker task notification channel is not initialized"
            );
            false
        }
    }

    ///

    /// # Arguments

    /// Borrows shared state.

    ///

    /// # Returns

    /// Whether an attached task sender still has a receiver.

    ///

    /// # Examples

    /// ```

    /// async fn ready(state: &codetether_worker_http::WorkerServerState) {

    /// let ready = state.is_ready().await;

    /// }

    /// ```

    pub async fn is_ready(&self) -> bool {
        // Knative readiness should reflect whether this HTTP server can accept
        // task pushes. The SSE control-plane stream can reconnect independently.
        self.task_notification_tx
            .lock()
            .await
            .as_ref()
            .is_some_and(|tx| !tx.is_closed())
    }
}
