//! Shared serialization defaults.

use chrono::{DateTime, Utc};

/// Helper to get current UTC time
pub(crate) fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

/// Default unit value
pub(crate) fn default_unit() -> String {
    "%".to_string()
}
