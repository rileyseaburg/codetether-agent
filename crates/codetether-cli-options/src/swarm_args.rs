//! CLI syntax for SwarmArgs.
use clap::Parser;
/// Parsed options for the swarm args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::SwarmArgs::try_parse_from(["swarm","Task"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug)]
pub struct SwarmArgs {
    /// Task to execute with swarm
    pub task: String,

    /// Model to use (provider/model format, e.g. zai/glm-5 or openrouter/z-ai/glm-5)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Decomposition strategy: auto, domain, data, stage, none
    #[arg(short = 's', long, default_value = "auto")]
    pub strategy: String,

    /// Maximum number of concurrent sub-agents
    #[arg(long, default_value = "100")]
    pub max_subagents: usize,

    /// Maximum steps per sub-agent
    #[arg(long, default_value = "100")]
    pub max_steps: usize,

    /// Timeout per sub-agent (seconds)
    #[arg(long, default_value = "300")]
    pub timeout: u64,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Sub-agent execution mode: local | k8s
    #[arg(long, default_value = "local", value_parser = ["local", "k8s", "kubernetes", "kubernetes-pod", "pod"])]
    pub execution_mode: String,

    /// Maximum concurrent Kubernetes sub-agent pods when execution mode is k8s.
    #[arg(long, default_value = "8")]
    pub k8s_pod_budget: usize,

    /// Optional image override for Kubernetes sub-agent pods.
    #[arg(long)]
    pub k8s_image: Option<String>,
}
