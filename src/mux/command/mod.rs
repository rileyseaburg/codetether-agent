//! CLI lifecycle operations for mux servers.

pub(in crate::mux) mod kill_all;
pub(in crate::mux) mod spawn;
pub(in crate::mux) mod startup;

mod host;

pub(super) async fn execute(command: codetether_mux_cli::MuxCommand) -> anyhow::Result<()> {
    codetether_mux_cli::execute(command, &host::Host).await
}
