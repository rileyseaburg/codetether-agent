//! Execute forage opportunities through run, swarm or Go engines.
//!
//! Dispatch captures before/after Git evidence and applies existing quality
//! gates. Engine modules preserve their timeouts, configuration and result
//! reporting. [Host] binds application execution, provider loading and policy.
use anyhow::Result;
use codetether_cli_options::ForageArgs;
use codetether_forage_core::{ForageOpportunity, execution::ExecutionOutcome};
use codetether_okr::OkrRepository;
mod execution;
mod go_exec;
mod host;
mod quality_shell;
mod run_exec;
/// Convert forage options into swarm limits and decomposition strategy.
pub mod swarm_config;
mod swarm_execution;
pub use execution::{execute_opportunity, run_quality_gates};
use go_exec::execute_opportunity_with_go;
pub use host::{Host, Providers, Swarm};
use run_exec::execute_opportunity_with_run;
use swarm_config::{build_swarm_config, parse_swarm_strategy};
use swarm_execution::execute_opportunity_with_swarm;
