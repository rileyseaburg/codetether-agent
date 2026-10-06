//! Typed denial returned when another agent owns a live mutation lease.

use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
#[error("Kubernetes mutation denied: {resource} is owned by {holder} until {expires_at}")]
pub struct LeaseConflict {
    pub resource: String,
    pub holder: String,
    pub expires_at: DateTime<Utc>,
}
