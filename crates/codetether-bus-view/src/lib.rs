//! Protocol bus-log entries, filtering and terminal presentation.
//!
//! [BusLogState] retains [BusLogEntry] values projected from the shared bus.
//! [render_bus_log] and [render_bus_log_with_summary] display those entries
//! with optional [ProtocolSummary] status, using shared theme and retention.
include!("entry_modules.rs");
include!("render_modules.rs");
include!("state_modules.rs");
pub use entry::BusLogEntry;
pub use render::{render_bus_log, render_bus_log_with_summary};
pub use state::BusLogState;
pub use summary::ProtocolSummary;

/// Cursor-based bus ingestion and safe external-message admission.
pub mod ingress;
