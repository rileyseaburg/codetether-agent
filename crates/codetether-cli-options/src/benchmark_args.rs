//! CLI syntax for BenchmarkArgs.
use clap::Parser;
/// Parsed options for the benchmark args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::BenchmarkArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct BenchmarkArgs {
    /// Directory containing benchmark PRD files
    #[arg(long, default_value = "benchmarks")]
    pub prd_dir: String,

    /// Models to benchmark (comma-separated, format: provider:model)
    #[arg(short, long, value_delimiter = ',')]
    pub models: Vec<String>,

    /// Only run PRDs matching this tier (1, 2, or 3)
    #[arg(long)]
    pub tier: Option<u8>,

    /// Run model×PRD combos in parallel
    #[arg(long)]
    pub parallel: bool,

    /// Maximum iterations per story
    #[arg(long, default_value = "10")]
    pub max_iterations: usize,

    /// Timeout per story in seconds
    #[arg(long, default_value = "300")]
    pub story_timeout: u64,

    /// Output file path
    #[arg(short, long, default_value = "benchmark_results.json")]
    pub output: String,

    /// Cost ceiling per run in USD (prevents runaway spending)
    #[arg(long, default_value = "50.0")]
    pub cost_ceiling: f64,

    /// Submit results to this API URL
    #[arg(long)]
    pub submit_url: Option<String>,

    /// API key for submitting results (Bearer token)
    #[arg(long, env = "BENCHMARK_API_KEY")]
    pub submit_key: Option<String>,

    /// Output as JSON to stdout
    #[arg(long)]
    pub json: bool,
}
