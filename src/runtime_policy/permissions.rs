//! Explicit `[permissions]` rule evaluation.

use super::ToolPolicyOutcome;
use crate::config::{PermissionAction, PermissionConfig};

pub(super) fn tool_outcome(
    permissions: &PermissionConfig,
    tool_name: &str,
) -> Option<ToolPolicyOutcome> {
    let action = permissions
        .tools
        .get(tool_name)
        .or_else(|| permissions.rules.get(tool_name))?;
    Some(match action {
        PermissionAction::Allow => ToolPolicyOutcome::Allow,
        PermissionAction::Deny => ToolPolicyOutcome::Deny,
        PermissionAction::Ask => ToolPolicyOutcome::RequireApproval,
    })
}
