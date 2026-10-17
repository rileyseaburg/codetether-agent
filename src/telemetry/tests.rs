//! Preserve shared globals and the application's report identity across the crate boundary.

#[test]
fn facade_shares_globals_and_registers_application_version() {
    super::alloc_guard_config::configure(std::env::temp_dir());
    assert_eq!(
        codetether_telemetry::application_version(),
        env!("CARGO_PKG_VERSION")
    );
    assert!(std::sync::Arc::ptr_eq(
        &super::TOKEN_USAGE,
        &codetether_telemetry::TOKEN_USAGE
    ));
    assert!(std::sync::Arc::ptr_eq(
        &super::TOOL_EXECUTIONS,
        &codetether_telemetry::TOOL_EXECUTIONS
    ));
    assert!(std::sync::Arc::ptr_eq(
        &super::PROVIDER_METRICS,
        &codetether_telemetry::PROVIDER_METRICS
    ));
    assert!(std::sync::Arc::ptr_eq(
        &super::TRANSPORT_METRICS,
        &codetether_telemetry::TRANSPORT_METRICS
    ));
}
