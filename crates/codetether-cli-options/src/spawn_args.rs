//! CLI syntax for SpawnArgs.
use clap::Parser;
/// Parsed options for the spawn args command.
/// # Examples
/// ```
/// use clap::Parser;
/// let parsed = codetether_cli_options::SpawnArgs::try_parse_from(["command"]);
/// assert!(parsed.is_ok());
/// ```

#[derive(Parser, Debug, Clone)]
pub struct SpawnArgs {
    /// Agent name. Defaults to <host>-<repo>-<short-pid>.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Hostname to bind. The zero-config default is reachable on the LAN and
    /// enables mDNS collaboration. Use 127.0.0.1 to opt into loopback-only.
    #[arg(long, default_value = "0.0.0.0")]
    pub hostname: String,

    /// Port to bind. 0 (default) lets the OS pick an available port.
    /// Specify a port if you need a stable URL for curl scripts.
    #[arg(short, long, default_value = "0")]
    pub port: u16,

    /// Public URL published in the agent card (defaults to http://<hostname>:<port>).
    #[arg(long)]
    pub public_url: Option<String>,

    /// Optional custom agent description for the card
    #[arg(short, long)]
    pub description: Option<String>,

    /// Explicit peer seed URLs (in addition to mDNS-discovered peers).
    /// Useful for cross-host setups where mDNS isn't routable.
    #[arg(long, value_delimiter = ',', env = "CODETETHER_A2A_PEERS")]
    pub peer: Vec<String>,

    /// Discovery interval for explicit --peer seeds, in seconds. Clamped
    /// to ≥ 5. (mDNS discovery is event-driven, not polled.)
    #[arg(long, default_value = "15")]
    pub discovery_interval_secs: u64,

    /// Disable sending an automatic intro message to newly discovered peers
    #[arg(long = "no-auto-introduce", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub auto_introduce: bool,

    /// Disable mDNS announce + browse. Without mDNS, only explicit
    /// --peer seeds are discovered. mDNS is on by default for true P2P.
    #[arg(long = "no-mdns", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub mdns: bool,
}
