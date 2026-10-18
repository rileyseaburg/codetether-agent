//! Forage startup, opportunity discovery and bounded execution cycles.
//!
//! [execute_with_summary] owns live repository, bus and optional archival
//! services through the run. Focused modules handle discovery, execution
//! counters, progress evidence, cycle limits and sink health. [Host] supplies
//! application scoring, interactive clarification and engine bindings.
use anyhow::Result;
use codetether_bus_core::{AgentBus, BusHandle};
use codetether_cli_options::ForageArgs;
use codetether_forage_core::seed::{
    seed_initial_okr_if_empty, seed_moonshot_okr_if_no_opportunities,
};
use codetether_forage_core::{
    ForageOpportunity, ForageRunSummary, MoonshotRubric, execution::ExecutionOutcome,
};
use codetether_forage_reporting as events;
use codetether_forage_reporting::{ensure_audit_log_initialized, flush_bus_observer};
use codetether_okr::{Okr, OkrRepository};
mod discover;
mod execute_selected;
mod execution_success;
mod host;
mod opportunities;
mod progress_adapter;
mod runtime;
mod s3_health;
mod s3_optional;
mod start_sink;
mod startup;
pub use host::Host;
use opportunities::{build_opportunities, load_moonshot_rubric};
pub use opportunities::{collect_opportunities, collect_opportunities_with_rubric};
pub use progress_adapter::record_execution_success_to_okr;
use s3_health::ensure_s3_sync_alive;
pub use startup::{execute, execute_with_summary};
/// Worker task metadata conversion, execution and summary rendering.
pub mod worker_task;
