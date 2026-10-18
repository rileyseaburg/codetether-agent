//! hooks config in shell hook-bypass detection.

pub(super) fn hooks_config(value: &str) -> bool {
    value.to_ascii_lowercase().starts_with("core.hookspath")
}
