//! Supply application metadata before telemetry startup.

pub(super) fn register() {
    codetether_telemetry::set_application_version(env!("CARGO_PKG_VERSION"));
}
