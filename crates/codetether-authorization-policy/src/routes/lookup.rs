//! Route matching and fail-closed permission requirements.
use super::POLICY_RULE_GROUPS;
use axum::http::StatusCode;

/// Find the required permission for a given path + method.
/// Returns `Some("")` for exempt, `Some(perm)` for required, `None` for unmatched.
/// # Arguments
/// * `path` - Request URI path.
/// * `method` - HTTP method name.
/// # Returns
/// First matching permission, an empty exemption, or None.
/// # Examples
/// ```
/// assert_eq!(codetether_authorization_policy::routes::match_policy_rule("/health", "GET"), Some(""));
/// ```
pub fn match_policy_rule(path: &str, method: &str) -> Option<&'static str> {
    for rule in POLICY_RULE_GROUPS.iter().flat_map(|rules| rules.iter()) {
        let matches = if rule.pattern.ends_with('/') {
            path.starts_with(rule.pattern)
        } else {
            path == rule.pattern || path.starts_with(&format!("{}/", rule.pattern))
        };
        if matches {
            if let Some(allowed_methods) = rule.methods
                && !allowed_methods.contains(&method)
            {
                continue;
            }
            return Some(rule.permission);
        }
    }
    None
}

/// # Arguments
/// * `path` - Request URI path.
/// * `method` - HTTP method name.
/// # Returns
/// Optional required permission; None marks an explicitly exempt route.
/// # Errors
/// Returns HTTP forbidden for an unmapped path or method.
/// # Examples
/// ```
/// assert!(codetether_authorization_policy::routes::required_permission("/unmapped", "GET").is_err());
/// ```
pub fn required_permission(path: &str, method: &str) -> Result<Option<&'static str>, StatusCode> {
    match match_policy_rule(path, method) {
        Some("") => Ok(None),
        Some(permission) => Ok(Some(permission)),
        None => Err(StatusCode::FORBIDDEN),
    }
}
