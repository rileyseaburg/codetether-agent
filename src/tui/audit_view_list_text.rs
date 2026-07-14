use crate::audit::AuditCategory;

pub(super) fn category(value: AuditCategory) -> &'static str {
    match value {
        AuditCategory::Api => "api",
        AuditCategory::ToolExecution => "tool",
        AuditCategory::Session => "session",
        AuditCategory::Cognition => "cognition",
        AuditCategory::Swarm => "swarm",
        AuditCategory::Auth => "auth",
        AuditCategory::K8s => "k8s",
        AuditCategory::Sandbox => "sandbox",
        AuditCategory::Config => "config",
    }
}

pub(super) fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        return value.to_string();
    }
    let cut: String = value.chars().take(max.saturating_sub(1)).collect();
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    #[test]
    fn truncate_is_unicode_safe() {
        assert_eq!(super::truncate("éééé", 3), "éé…");
    }
}
