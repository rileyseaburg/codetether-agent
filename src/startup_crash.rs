//! Bind shared crash startup to this application's version and memory guards.
pub(super) async fn initialize(is_tui: bool) {
    codetether_cli_startup::crash::initialize(
        is_tui,
        env!("CARGO_PKG_VERSION"),
        codetether_agent::telemetry::install_memory_guards,
    )
    .await;
}
