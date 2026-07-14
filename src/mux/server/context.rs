//! Shared mux server state and persistence.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, Notify, RwLock};

use crate::mux::model::MuxSnapshot;
use crate::mux::pty::PtyRegistry;
use crate::mux::registry::{self, MuxRecord};

pub(super) struct ServerContext {
    pub state: RwLock<MuxSnapshot>,
    pub token: String,
    pub address: SocketAddr,
    pub started_at: DateTime<Utc>,
    pub shutdown: Notify,
    pub programs: PtyRegistry,
    persist_lock: Mutex<()>,
}

impl ServerContext {
    pub(super) fn new(state: MuxSnapshot, token: String, address: SocketAddr) -> Arc<Self> {
        Arc::new(Self {
            state: RwLock::new(state),
            token,
            address,
            started_at: Utc::now(),
            shutdown: Notify::new(),
            programs: PtyRegistry::new(),
            persist_lock: Mutex::new(()),
        })
    }

    pub(super) async fn persist(&self) -> Result<()> {
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
