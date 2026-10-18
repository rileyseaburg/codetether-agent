//! CLI syntax for ForageArgs.
use clap::Parser;
use std::path::PathBuf;
/// Parsed options for the forage args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::ForageArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
#[command(
    about = "OKR-governed autonomous opportunity scanner/executor",
    long_about = "Scan OKRs/KRs for high-priority opportunities and optionally execute the top selections with the build agent.",
    after_long_help = "Examples:\n  codetether forage --top 5\n  codetether forage --loop --interval-secs 600\n  codetether forage --loop --execute --top 1 --interval-secs 600 --max-cycles 48 --run-timeout-secs 900 --model minimax/MiniMax-M2.5\n  codetether forage --loop --execute --execution-engine swarm --swarm-max-subagents 8 --swarm-strategy auto --model zai/glm-5"
)]
pub struct ForageArgs {
    /// Show top-N opportunities each cycle
    #[arg(long, default_value = "3")]
    pub top: usize,

    /// Keep running continuously
    #[arg(long = "loop")]
    pub loop_mode: bool,

    /// Seconds between loop cycles
    #[arg(long, default_value = "120")]
    pub interval_secs: u64,

    /// Maximum cycles when looping (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub max_cycles: usize,

    /// Execute selected opportunities via `codetether run`
    #[arg(long)]
    pub execute: bool,

    /// Disable S3/MinIO archival requirement (for local-only execution)
    #[arg(long)]
    pub no_s3: bool,

    /// Moonshot mission statement(s) used as a strategic rubric for prioritization.
    ///
    /// Repeat the flag to provide multiple missions.
    #[arg(long = "moonshot")]
    pub moonshots: Vec<String>,

    /// Optional file containing moonshot mission statements (JSON array or newline-delimited text).
    #[arg(long)]
    pub moonshot_file: Option<PathBuf>,

    /// Require minimum moonshot alignment for an opportunity to be selected.
    #[arg(long)]
    pub moonshot_required: bool,

    /// Minimum moonshot alignment ratio when `--moonshot-required` is enabled.
    #[arg(long, default_value = "0.10")]
    pub moonshot_min_alignment: f64,

    /// Execution engine for `--execute`: run | swarm | go
    #[arg(long, default_value = "run", value_parser = ["run", "swarm", "go"])]
    pub execution_engine: String,

    /// Timeout per execution task in seconds (only used with --execute)
    #[arg(long, default_value = "900")]
    pub run_timeout_secs: u64,

    /// Stop immediately on execution error/timeout (default: continue loop)
    #[arg(long)]
    pub fail_fast: bool,

    /// Swarm decomposition strategy when --execution-engine=swarm
    #[arg(long, default_value = "auto", value_parser = ["auto", "domain", "data", "stage", "none"])]
    pub swarm_strategy: String,

    /// Maximum concurrent sub-agents when --execution-engine=swarm
    #[arg(long, default_value = "8")]
    pub swarm_max_subagents: usize,

    /// Maximum steps per sub-agent when --execution-engine=swarm
    #[arg(long, default_value = "100")]
    pub swarm_max_steps: usize,

    /// Timeout per sub-agent in seconds when --execution-engine=swarm
    #[arg(long, default_value = "300")]
    pub swarm_subagent_timeout_secs: u64,

    /// Optional model override for execution mode
    #[arg(short, long)]
    pub model: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}
