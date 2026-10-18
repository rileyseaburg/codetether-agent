//! Initialize counters for one forage run.
use super::*;
pub(super) fn initial(args: &ForageArgs) -> ForageRunSummary {
    ForageRunSummary {
        cycles_completed: 0,
        execute_requested: args.execute,
        execution_engine: args.execution_engine.clone(),
        total_selected: 0,
        total_executed: 0,
        total_execution_failures: 0,
        last_cycle_selected: Vec::new(),
    }
}
