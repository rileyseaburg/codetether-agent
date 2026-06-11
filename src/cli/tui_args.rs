use crate::config::AccessMode;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct TuiArgs {
    /// Project directory
    pub project: Option<PathBuf>,
    /// Transient access mode: ask, approve, or full
    #[arg(long, value_parser = clap::value_parser!(AccessMode))]
    pub access_mode: Option<AccessMode>,
    /// Allow network access in sandboxed commands
    #[arg(long)]
    pub allow_network: bool,
    /// Disable the built-in A2A peer endpoint.
    #[arg(long = "no-a2a", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub a2a: bool,
    /// Override the auto-picked A2A port.
    #[arg(long)]
    pub a2a_port: Option<u16>,
    /// A2A bind hostname.
    #[arg(long, default_value = "0.0.0.0")]
    pub a2a_hostname: String,
    /// Public URL published in the agent card.
    #[arg(long)]
    pub a2a_public_url: Option<String>,
    /// Override the auto-picked agent name.
    #[arg(long)]
    pub a2a_name: Option<String>,
    /// Optional description for the A2A card.
    #[arg(long)]
    pub a2a_description: Option<String>,
    /// Explicit peer seed URLs.
    #[arg(long, value_delimiter = ',', env = "CODETETHER_A2A_PEERS")]
    pub a2a_peer: Vec<String>,
    /// Discovery interval for explicit --a2a-peer seeds, in seconds.
    #[arg(long, default_value = "15")]
    pub a2a_discovery_interval_secs: u64,
    /// Disable auto-intro to newly discovered peers.
    #[arg(long = "a2a-no-auto-introduce", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub a2a_auto_introduce: bool,
    /// Disable mDNS-based peer discovery.
    #[arg(long = "a2a-no-mdns", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub a2a_mdns: bool,
}
