//! Worker state identity operations.
use super::*;
impl WorkerServerState {
    ///
    /// # Arguments
    /// connected is the control-plane connection flag.
    ///
    /// # Returns
    /// Completes after replacing the flag.
    ///
    /// # Examples
    /// ```
    /// async fn update(state: &codetether_worker_http::WorkerServerState) {
    /// state.set_connected(true).await;
    /// }
    /// ```
    pub async fn set_connected(&self, connected: bool) {
        *self.connected.lock().await = connected;
    }

    ///

    /// # Arguments

    /// worker_id is the registered worker identifier.

    ///

    /// # Returns

    /// Completes after replacing the identifier.

    ///

    /// # Examples

    /// ```

    /// async fn update(state: &codetether_worker_http::WorkerServerState) {

    /// state.set_worker_id("w1".into()).await;

    /// }

    /// ```

    pub async fn set_worker_id(&self, worker_id: String) {
        *self.worker_id.lock().await = Some(worker_id);
    }

    #[allow(dead_code)]
    ///
    /// # Arguments
    /// Borrows shared state.
    ///
    /// # Returns
    /// The registered identifier, or an empty string.
    ///
    /// # Examples
    /// ```
    /// async fn read(state: &codetether_worker_http::WorkerServerState) {
    /// let id = state.worker_id().await;
    /// }
    /// ```
    pub async fn worker_id(&self) -> String {
        self.worker_id.lock().await.clone().unwrap_or_default()
    }

    #[allow(dead_code)]
    ///
    /// # Arguments
    /// Borrows shared state.
    ///
    /// # Returns
    /// The last supplied connection flag.
    ///
    /// # Examples
    /// ```
    /// async fn read(state: &codetether_worker_http::WorkerServerState) {
    /// let connected = state.is_connected().await;
    /// }
    /// ```
    pub async fn is_connected(&self) -> bool {
        *self.connected.lock().await
    }
}
