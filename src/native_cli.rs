//! Native commands must not initialize Vault, sessions or agent infrastructure.
use codetether_agent::cli::{self, Command};
pub(crate) async fn dispatch(command: &Option<Command>) -> Option<anyhow::Result<()>> {
    match command {
        Some(Command::Windows(args)) => Some(cli::windows::run(args.clone()).await),
        Some(Command::Bonsai(args)) => Some(cli::bonsai::run(args.clone()).await),
        _ => None,
    }
}
