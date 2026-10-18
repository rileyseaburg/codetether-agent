//! Forage lifecycle reporting through the console, agent bus and audit trail.
//!
//! [RunEvents] borrows the existing run options, rubric and bus handle. Its
//! methods publish lifecycle and execution outcomes. [console] owns the one
//! shared quiet flag used by CLI and TUI callers; [flush_bus_observer] drains
//! the existing bounded observation stream. Audit initialization and correlated
//! records use the shared application audit singleton.
use anyhow::Result;
use codetether_bus_core::{BusHandle, BusMessage};
use codetether_cli_options::ForageArgs;
use codetether_forage_core::{ForageOpportunity, MoonshotRubric, execution::ExecutionOutcome};
mod audit_adapter;
/// Shared quiet-mode state and human-readable cycle output.
pub mod console;
mod console_entry;
mod events;
mod observer;
pub use audit_adapter::{ensure_audit_log_initialized, log_audit};
pub use events::RunEvents;
pub use observer::flush_bus_observer;
