//! CLI lifecycle operations for mux servers.

#[cfg(test)]
pub(in crate::mux) mod kill_all;

mod host;

pub(super) async fn execute(command: codetether_mux_cli::MuxCommand) -> anyhow::Result<()> {
    codetether_mux_cli::execute(command, &host::Host).await
}
