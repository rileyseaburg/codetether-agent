//! Worker startup mapping from auto-approval to process access mode.
use crate::AccessMode;

/// Map a worker auto-approve policy onto the equivalent runtime access mode.
///
/// Only the unattended `all` policy widens the access mode; `safe` and `none`
/// leave the configured value untouched so their prompts still apply.
/// # Arguments
/// `auto_approve` is the worker's configured approval policy string.
/// # Returns
/// An explicit full-access override only for the trimmed value `all`.
/// # Examples
/// ```
/// assert!(codetether_config::a2a_runtime::auto_approve_access_mode("safe").is_none());
/// ```
pub fn auto_approve_access_mode(auto_approve: &str) -> Option<AccessMode> {
    match auto_approve.trim() {
        "all" => Some(AccessMode::Full),
        _ => None,
    }
}

#[cfg(test)]
mod auto_approve_access_mode_tests {
    use super::auto_approve_access_mode;
    use crate::AccessMode;

    #[test]
    fn unattended_all_policy_grants_full_access() {
        assert!(matches!(
            auto_approve_access_mode("all"),
            Some(AccessMode::Full)
        ));
        assert!(matches!(
            auto_approve_access_mode(" all "),
            Some(AccessMode::Full)
        ));
    }

    #[test]
    fn narrower_policies_keep_configured_access_mode() {
        assert!(auto_approve_access_mode("safe").is_none());
        assert!(auto_approve_access_mode("none").is_none());
        assert!(auto_approve_access_mode("").is_none());
    }
}
