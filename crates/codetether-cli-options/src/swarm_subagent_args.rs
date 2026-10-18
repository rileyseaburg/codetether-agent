//! Args for the hidden `swarm-subagent` command.

use clap::Parser;

/// Payload source for executing a single swarm subtask in a sub-process.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::SwarmSubagentArgs::try_parse_from(["swarm-subagent"]);
/// assert!(parsed.is_ok());
/// ```
#[derive(Parser, Debug)]
pub struct SwarmSubagentArgs {
    /// Base64 payload for the remote subtask (JSON encoded)
    #[arg(long)]
    pub payload_base64: Option<String>,

    /// Read payload from an environment variable
    #[arg(long, default_value = "CODETETHER_SWARM_SUBTASK_PAYLOAD")]
    pub payload_env: String,
}
