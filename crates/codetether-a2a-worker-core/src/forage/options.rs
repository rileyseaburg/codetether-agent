//! Forage execution settings parsed from worker task metadata.
use std::path::PathBuf;
/// Typed forage options independent of the application's command-line parser.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::forage::build_forage_options;
/// let options = build_forage_options("mission", "title", &serde_json::Map::new(), None);
/// assert_eq!(options.moonshots, ["mission"]);
/// ```

#[derive(Debug, Clone)]
pub struct ForageTaskOptions {
    /// Show top-N opportunities each cycle
    pub top: usize,

    /// Keep running continuously
    pub loop_mode: bool,

    /// Seconds between loop cycles
    pub interval_secs: u64,

    /// Maximum cycles when looping (0 = unlimited)
    pub max_cycles: usize,

    /// Execute selected opportunities via `codetether run`
    pub execute: bool,

    /// Disable S3/MinIO archival requirement (for local-only execution)
    pub no_s3: bool,

    /// Moonshot mission statement(s) used as a strategic rubric for prioritization.
    ///
    /// Repeat the flag to provide multiple missions.
    pub moonshots: Vec<String>,

    /// Optional file containing moonshot mission statements (JSON array or newline-delimited text).
    pub moonshot_file: Option<PathBuf>,

    /// Require minimum moonshot alignment for an opportunity to be selected.
    pub moonshot_required: bool,

    /// Minimum moonshot alignment ratio when `--moonshot-required` is enabled.
    pub moonshot_min_alignment: f64,

    /// Execution engine for `--execute`: run | swarm | go
    pub execution_engine: String,

    /// Timeout per execution task in seconds (only used with --execute)
    pub run_timeout_secs: u64,

    /// Stop immediately on execution error/timeout (default: continue loop)
    pub fail_fast: bool,

    /// Swarm decomposition strategy when --execution-engine=swarm
    pub swarm_strategy: String,

    /// Maximum concurrent sub-agents when --execution-engine=swarm
    pub swarm_max_subagents: usize,

    /// Maximum steps per sub-agent when --execution-engine=swarm
    pub swarm_max_steps: usize,

    /// Timeout per sub-agent in seconds when --execution-engine=swarm
    pub swarm_subagent_timeout_secs: u64,

    /// Optional model override for execution mode
    pub model: Option<String>,

    /// Output as JSON
    pub json: bool,
}
