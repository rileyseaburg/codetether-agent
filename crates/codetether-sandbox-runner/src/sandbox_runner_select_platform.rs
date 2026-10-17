//! Platform-specific reason reported when no OS sandbox backend applies.

/// Reason code for a host with no compiled-in sandbox backend.
///
/// Linux uses bubblewrap or Landlock and macOS uses Seatbelt, so this reason
/// only surfaces on other targets (notably Windows, which has no supported
/// confinement backend yet).
pub(super) fn unsupported_reason() -> &'static str {
    reason_for(std::env::consts::OS)
}

fn reason_for(os: &str) -> &'static str {
    match os {
        "macos" => "sandbox_exec_not_found",
        "windows" => "no_sandbox_backend_on_windows",
        _ => "no_sandbox_backend_on_this_platform",
    }
}

#[cfg(test)]
mod tests {
    use super::reason_for;

    #[test]
    fn each_platform_reports_an_actionable_reason() {
        assert_eq!(reason_for("macos"), "sandbox_exec_not_found");
        assert_eq!(reason_for("windows"), "no_sandbox_backend_on_windows");
        assert_eq!(reason_for("freebsd"), "no_sandbox_backend_on_this_platform");
    }
}
