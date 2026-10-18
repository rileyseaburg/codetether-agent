//! Application scoring and interactive bindings for the shared forage runtime.
use crate::cli::ForageArgs;
use anyhow::Result;
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};
pub(crate) mod execution_host;
mod startup;
use codetether_forage_clarify as clarify;
pub use startup::{execute, execute_with_summary};
pub mod console;
#[cfg(test)]
use crate::okr::Okr;
#[cfg(test)]
use codetether_forage_core::{ForageOpportunity, MoonshotRubric, execution::ExecutionOutcome};
#[cfg(test)]
mod opportunities;
#[cfg(test)]
use codetether_forage_runtime as progress_adapter;
#[cfg(test)]
use opportunities::{collect_opportunities, collect_opportunities_with_rubric};
#[cfg(test)]
use progress_adapter::record_execution_success_to_okr;
#[cfg(test)]
mod swarm_config;
#[cfg(test)]
use swarm_config::build_swarm_config;
#[cfg(test)]
mod tests;
