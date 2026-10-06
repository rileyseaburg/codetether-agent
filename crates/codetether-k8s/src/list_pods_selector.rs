//! Pure label-selector resolution for pod listing (unit-testable).

/// Resolve the effective label selector for a pod list.
///
/// Precedence: a non-blank explicit `sel` wins; otherwise fall back to the
/// deployment scope as `app=<deployment>`; otherwise `None` (list all pods,
/// no hidden filter).
pub(super) fn resolve_selector(sel: Option<&str>, deployment: Option<&str>) -> Option<String> {
    sel.filter(|v| !v.trim().is_empty())
        .map(ToString::to_string)
        .or_else(|| deployment.map(|n| format!("app={n}")))
}

#[cfg(test)]
mod tests {
    use super::resolve_selector;

    #[test]
    fn explicit_selector_wins() {
        assert_eq!(
            resolve_selector(Some("tier=web"), Some("api")),
            Some("tier=web".to_string())
        );
    }

    #[test]
    fn blank_selector_falls_back_to_deployment() {
        assert_eq!(
            resolve_selector(Some("  "), Some("api")),
            Some("app=api".to_string())
        );
    }

    #[test]
    fn no_selector_no_deployment_lists_all() {
        // The historical bug: this used to default to `app=codetether`,
        // silently hiding every unlabeled pod. It must now be `None`.
        assert_eq!(resolve_selector(None, None), None);
    }
}
