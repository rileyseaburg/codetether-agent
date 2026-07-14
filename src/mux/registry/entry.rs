//! Persistent connection information for one mux server.

use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mux::model::MuxSnapshot;

/// Private discovery record stored with owner-only permissions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::mux) struct MuxRecord {
    pub name: String,
    pub address: SocketAddr,
    pub token: String,
    pub pid: u32,
    pub started_at: DateTime<Utc>,
    pub state: MuxSnapshot,
}
