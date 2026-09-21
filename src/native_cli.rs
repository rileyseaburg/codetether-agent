//! Native commands must not initialize Vault, sessions or agent infrastructure.
use clap::Parser;
use codetether_agent::cli::{self, Cli, Command};
pub(crate) async fn early() -> Option<anyhow::Result<()>> {
    if !std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "windows" || arg == "bonsai")
    {
        return None;
    }
    dispatch(&Cli::parse().command).await
}
pub(crate) async fn dispatch(command: &Option<Command>) -> Option<anyhow::Result<()>> {
    match command {
        Some(Command::Windows(args)) => Some(cli::windows::run(args.clone()).await),
        Some(Command::Bonsai(args)) => Some(cli::bonsai::run(args.clone()).await),
        _ => None,
    }
}
