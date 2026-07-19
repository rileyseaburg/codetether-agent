//! Durable mux discovery-record persistence.

use super::context::ServerContext;
use crate::mux::registry::{self, MuxRecord};

impl ServerContext {
    pub(super) async fn persist(&self) -> anyhow::Result<()> {
        let _guard = self.persist_lock.lock().await;
        let state = self.state.read().await.clone();
        registry::store(&MuxRecord {
            name: state.name.clone(),
            address: self.address,
            token: self.token.clone(),
            pid: std::process::id(),
            started_at: self.started_at,
            state,
        })
        .await
    }
}
