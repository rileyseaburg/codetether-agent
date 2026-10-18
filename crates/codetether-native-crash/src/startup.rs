//! Bind shared crash startup to this application's version and memory guards.
/// Initialize crash reporting for CLI startup with the native application identity.
pub async fn initialize(is_tui: bool) {
    codetether_cli_startup::crash::initialize(
        is_tui,
        env!("CARGO_PKG_VERSION"),
        codetether_native_telemetry::install_memory_guards,
    )
    .await;
}
