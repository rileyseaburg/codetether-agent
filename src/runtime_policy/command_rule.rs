//! Configured command-prefix permission rules.

use crate::config::{PermissionAction, PermissionConfig};

use super::ToolPolicyOutcome;

pub(super) fn outcome(permissions: &PermissionConfig, command: &str) -> Option<ToolPolicyOutcome> {
    permissions
        .rules
        .iter()
        .filter(|(prefix, _)| command.trim_start().starts_with(prefix.as_str()))
        .max_by_key(|(prefix, _)| prefix.len())
        .map(|(_, action)| match action {
            PermissionAction::Allow => ToolPolicyOutcome::Allow,
            PermissionAction::Deny => ToolPolicyOutcome::Deny,
            PermissionAction::Ask => ToolPolicyOutcome::RequireApproval,
        })
}
