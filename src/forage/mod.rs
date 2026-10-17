//! Application execution and event adapters for forage work selection.
use codetether_forage_core::seed::{
    seed_initial_okr_if_empty, seed_moonshot_okr_if_no_opportunities,
};

use codetether_forage_core::{ForageOpportunity, MoonshotRubric};
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};

use crate::bus::{AgentBus, BusHandle, BusMessage};
use crate::cli::ForageArgs;
use crate::okr::{Okr, OkrRepository};
use run_exec::execute_opportunity_with_run;
mod s3_health;
use s3_health::ensure_s3_sync_alive;
mod progress_adapter;
use progress_adapter::record_execution_success_to_okr;
mod go_exec;
use go_exec::execute_opportunity_with_go;
mod discover;
mod events;
mod execute_selected;
mod execution;
mod execution_success;
mod startup;
pub use startup::{execute, execute_with_summary};
mod runtime;
mod start_sink;
use execution::execute_opportunity;
mod swarm_execution;
use swarm_execution::execute_opportunity_with_swarm;
mod swarm_config;
use swarm_config::{build_swarm_config, parse_swarm_strategy};
mod audit_adapter;
use audit_adapter::{ensure_audit_log_initialized, log_audit};
mod observer;
use observer::flush_bus_observer;
mod opportunities;
use opportunities::{build_opportunities, load_moonshot_rubric};
#[cfg(test)]
use opportunities::{collect_opportunities, collect_opportunities_with_rubric};
mod clarify;
pub mod console;
mod console_entry;
mod quality_shell;
mod run_exec;
mod s3_optional;
mod tetherscript_score;
use anyhow::Result;
use codetether_forage_core::execution::ExecutionOutcome;
#[cfg(test)]
mod tests;
