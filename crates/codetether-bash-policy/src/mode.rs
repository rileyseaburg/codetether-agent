//! Environment defaults and workspace root selection for bash execution.
//! Use `default_enabled` at tool construction and `root` when building a policy.

/// Resolve the initial sandbox setting from environment overrides.
///
/// # Arguments
/// None.
/// # Returns
/// Whether bash sandboxing is enabled by default.
///
/// # Examples
/// ```rust
/// let _enabled = codetether_bash_policy::mode::default_enabled();
/// ```
pub fn default_enabled() -> bool {
    resolve(
        std::env::var("CODETETHER_UNSANDBOXED_BASH").ok().as_deref(),
        std::env::var("CODETETHER_SANDBOX_BASH").ok().as_deref(),
    )
}

fn resolve(unsandboxed: Option<&str>, sandboxed: Option<&str>) -> bool {
    !unsandboxed.is_some_and(truthy) && sandboxed.map_or(true, parse_sandbox_flag)
}

fn parse_sandbox_flag(value: &str) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "0" | "false" | "no" | "off" => false,
        "1" | "true" | "yes" | "on" => true,
        _ => true,
    }
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

/// Describe the environment override that disables sandboxing.
///
/// # Arguments
/// None.
/// # Returns
/// The override reason, or the tool-default reason.
///
/// # Examples
/// ```rust
/// let reason = codetether_bash_policy::mode::unsafe_reason();
/// assert!(!reason.is_empty());
/// ```
pub fn unsafe_reason() -> &'static str {
    if env_truthy("CODETETHER_UNSANDBOXED_BASH") {
        "CODETETHER_UNSANDBOXED_BASH enabled"
    } else if std::env::var("CODETETHER_SANDBOX_BASH").is_ok_and(|v| !parse_sandbox_flag(&v)) {
        "CODETETHER_SANDBOX_BASH disabled"
    } else {
        "BashTool sandbox flag disabled"
    }
}

/// Choose an explicit working directory or the process working directory.
///
/// # Arguments
/// `cwd` is the optional requested working directory.
/// # Returns
/// The working directory, falling back to the temporary directory.
///
/// # Examples
/// ```rust
/// let root = codetether_bash_policy::mode::root(Some("/workspace".into()));
/// assert_eq!(root, std::path::PathBuf::from("/workspace"));
/// ```
pub fn root(cwd: Option<std::path::PathBuf>) -> std::path::PathBuf {
    cwd.or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(std::env::temp_dir)
}

fn env_truthy(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| truthy(&value))
}

#[cfg(test)]
mod tests {
    use super::resolve;

    #[test]
    fn sandboxing_enabled_by_default() {
        assert!(resolve(None, None));
    }

    #[test]
    fn explicit_unsandboxed_opt_out_disables_sandboxing() {
        assert!(!resolve(Some("1"), None));
    }
}
