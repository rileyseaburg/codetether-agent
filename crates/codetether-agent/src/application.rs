//! Native bindings for the shared CLI process lifecycle.
use clap::Parser;
use codetether_agent::cli::{Cli, Command};

pub(super) struct Native;
impl codetether_cli_startup::lifecycle::Host for Native {
    type Invocation = Cli;
    fn parse() -> Cli {
        Cli::parse()
    }
    fn command(invocation: &Cli) -> &Option<Command> {
        &invocation.command
    }
    async fn native(invocation: &Cli) -> Option<anyhow::Result<()>> {
        codetether_cli_startup::native::dispatch(&invocation.command).await
    }
    async fn initialize_crash(is_tui: bool) {
        super::startup_crash::initialize(is_tui).await;
    }
    async fn dispatch(invocation: Cli) -> anyhow::Result<()> {
        super::dispatch(invocation).await
    }
}
