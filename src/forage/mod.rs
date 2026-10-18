//! Application scoring and interactive bindings for the shared forage runtime.
use crate::cli::ForageArgs;
use anyhow::Result;
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};
mod execution_host;
mod startup;
pub use startup::{execute, execute_with_summary};
mod clarify;
pub mod console;
mod tetherscript_score;
#[cfg(test)]
use crate::okr::Okr;
#[cfg(test)]
use codetether_forage_core::{ForageOpportunity, MoonshotRubric, execution::ExecutionOutcome};
#[cfg(test)]
mod opportunities;
#[cfg(test)]
use opportunities::{collect_opportunities, collect_opportunities_with_rubric};
#[cfg(test)]
mod progress_adapter;
#[cfg(test)]
use progress_adapter::record_execution_success_to_okr;
#[cfg(test)]
mod swarm_config;
#[cfg(test)]
use swarm_config::build_swarm_config;
#[cfg(test)]
mod tests;
