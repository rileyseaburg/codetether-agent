//! Application scoring and interactive bindings for the shared forage runtime.
use crate::cli::ForageArgs;
use anyhow::Result;
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};
pub(crate) mod execution_host;
mod startup;
pub use startup::{execute, execute_with_summary};
pub mod console;
