//! Host identity carried by process-wide telemetry reports.

use std::sync::OnceLock;
static VERSION: OnceLock<&'static str> = OnceLock::new();

/// Set the host version before installing memory guards. The first call wins.
///
/// # Arguments
/// * `version` - Application package version, available for the process lifetime.
/// # Examples
/// ```
/// codetether_telemetry::set_application_version("1.2.3");
/// assert_eq!(codetether_telemetry::application_version(), "1.2.3");
/// ```
pub fn set_application_version(version: &'static str) {
    let _ = VERSION.set(version);
}

/// Return the host version, or this crate's version before host configuration.
///
/// # Returns
/// The configured static version string.
/// # Examples
/// ```
/// assert!(!codetether_telemetry::application_version().is_empty());
/// ```
pub fn application_version() -> &'static str {
    VERSION.get().copied().unwrap_or(env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn host_version_reaches_allocator_report() {
        super::set_application_version("test-host-version");
        let report = crate::alloc_guard_payload::build("report", 100, 50, "trace");
        assert_eq!(report["app_version"], "test-host-version");
    }
}
