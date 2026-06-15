//! Default `ForageArgs` construction for the TUI `/forage` command.
//!
//! Builds a sensible local-only forage configuration so the TUI can launch
//! a scan (or execution) run without requiring the user to type every flag.

use crate::cli::ForageArgs;

/// Build [`ForageArgs`] for a TUI forage run.
///
/// Scans only (no execution) by default; `execute` toggles the build agent.
/// Runs a single cycle and avoids the S3 archival requirement so it works in
/// local-only environments.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::forage_run::build_tui_forage_args;
///
/// let args = build_tui_forage_args(5, false, None);
/// assert_eq!(args.top, 5);
/// assert!(!args.execute);
/// assert!(args.no_s3);
/// ```
pub fn build_tui_forage_args(top: usize, execute: bool, model: Option<String>) -> ForageArgs {
    ForageArgs {
        top,
        loop_mode: false,
        interval_secs: 120,
        max_cycles: 1,
        execute,
        no_s3: true,
        moonshots: Vec::new(),
        moonshot_file: None,
        moonshot_required: false,
        moonshot_min_alignment: 0.10,
        execution_engine: "run".to_string(),
        run_timeout_secs: 900,
        fail_fast: false,
        swarm_strategy: "auto".to_string(),
        swarm_max_subagents: 8,
        swarm_max_steps: 100,
        swarm_subagent_timeout_secs: 300,
        model,
        json: false,
    }
}
