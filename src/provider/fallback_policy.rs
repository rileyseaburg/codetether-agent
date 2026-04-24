//! Provider credential fallback policy.

pub(crate) const DISABLE_ENV_FALLBACK: &str = "CODETETHER_DISABLE_ENV_FALLBACK";

pub(crate) fn env_fallback_disabled() -> bool {
    let value = std::env::var(DISABLE_ENV_FALLBACK).ok();
    env_fallback_disabled_from(value.as_deref())
}

pub(crate) fn env_fallback_disabled_from(value: Option<&str>) -> bool {
    value
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub(crate) fn registry_mode_label(disabled: bool) -> &'static str {
    if disabled {
        "Vault only"
    } else {
        "Vault + env/AWS fallback"
    }
}

#[cfg(test)]
mod tests {
    use super::{env_fallback_disabled_from, registry_mode_label};

    #[test]
    fn parses_security_hardened_flag() {
        assert!(env_fallback_disabled_from(Some("1")));
        assert!(env_fallback_disabled_from(Some("true")));
        assert!(env_fallback_disabled_from(Some("TRUE")));
        assert!(!env_fallback_disabled_from(Some("0")));
        assert!(!env_fallback_disabled_from(None));
    }

    #[test]
    fn labels_provider_registry_mode() {
        assert_eq!(registry_mode_label(true), "Vault only");
        assert_eq!(registry_mode_label(false), "Vault + env/AWS fallback");
    }
}
