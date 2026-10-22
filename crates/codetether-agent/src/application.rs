//! Native bindings for the shared CLI process lifecycle.
use clap::{CommandFactory, FromArgMatches};
use codetether_agent::cli::{Cli, Command};

pub(super) struct Native;
impl codetether_cli_startup::lifecycle::Host for Native {
    type Invocation = Cli;
    fn parse() -> Cli {
        Cli::from_arg_matches(&parser().get_matches()).unwrap_or_else(|error| error.exit())
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

fn parser() -> clap::Command {
    Cli::command()
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_comes_from_the_binary_package() {
        let error = super::parser()
            .try_get_matches_from(["codetether", "--version"])
            .unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::DisplayVersion);
        assert_eq!(
            error.to_string(),
            format!("codetether {}\n", env!("CARGO_PKG_VERSION"))
        );
    }
}
