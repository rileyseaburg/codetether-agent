//! Opt-in policy for legacy workspace-wide recall migration.

const ENV: &str = "CODETETHER_RECALL_BACKFILL";

pub(super) fn enabled() -> bool {
    enabled_from(std::env::var(ENV).ok().as_deref())
}

fn enabled_from(value: Option<&str>) -> bool {
    value.is_some_and(|value| matches!(value.trim(), "1" | "true" | "yes" | "on"))
}

#[cfg(test)]
mod tests {
    use super::enabled_from;

    #[test]
    fn legacy_backfill_is_disabled_unless_explicitly_enabled() {
        assert!(!enabled_from(None));
        assert!(!enabled_from(Some("false")));
        assert!(enabled_from(Some("1")));
        assert!(enabled_from(Some("yes")));
    }
}
