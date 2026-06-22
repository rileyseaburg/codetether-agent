//! CLI arguments for `codetether connect`.

use clap::Parser;

/// Arguments for `codetether connect`.
///
/// Connects to a designated Ubuntu VM over SSH, forwards a dedicated local
/// port to the remote auth callback, runs the remote device-code flow, and
/// opens the verification URL in the local browser.
#[derive(Parser, Debug)]
pub struct ConnectArgs {
    /// Hostname or IP of the Ubuntu VM on your network
    #[arg(long, env = "CODETETHER_CONNECT_HOST")]
    pub host: String,
    /// SSH username on the VM
    #[arg(long, short = 'u', env = "CODETETHER_CONNECT_USER")]
    pub user: Option<String>,
    /// SSH port on the VM
    #[arg(long, default_value_t = 22)]
    pub port: u16,
    /// Dedicated local+remote port to forward for the auth callback
    #[arg(long, default_value_t = 1455)]
    pub forward_port: u16,
    /// Path to the remote `codetether` binary
    #[arg(long, default_value = "codetether")]
    pub remote_bin: String,
    /// Remote auth subcommand to run (e.g. "bedrock")
    #[arg(long, default_value = "bedrock")]
    pub provider: String,
    /// Extra args passed verbatim to the remote `auth` command
    #[arg(long, value_delimiter = ' ', allow_hyphen_values = true)]
    pub remote_args: Vec<String>,
    /// Print the verification URL instead of opening a browser
    #[arg(long, default_value_t = false)]
    pub no_browser: bool,
    /// Skip the remote binary preflight check
    #[arg(long, default_value_t = false)]
    pub skip_preflight: bool,
}
