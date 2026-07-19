//! Shared mux server state and persistence.

use std::net::SocketAddr;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::{Mutex, Notify, RwLock};

use crate::mux::model::MuxSnapshot;
use crate::mux::pty::PtyRegistry;

pub(super) struct ServerContext {
    pub state: RwLock<MuxSnapshot>,
    pub token: String,
    pub address: SocketAddr,
    pub started_at: DateTime<Utc>,
    pub shutdown: Notify,
    pub programs: PtyRegistry,
    pub leases: crate::mux::lease::LeaseRegistry,
    pub(super) persist_lock: Mutex<()>,
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
            leases: crate::mux::lease::LeaseRegistry::new(),
            persist_lock: Mutex::new(()),
        })
    }
}
