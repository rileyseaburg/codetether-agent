//! Config-backed runtime tool policy.

use super::{DecisionReason, ToolKind, ToolPolicyDecision};
use crate::config::{ApprovalPolicy, Config, PermissionConfig, PermissionProfile, SandboxMode};

/// Reusable decision state derived from effective configuration.
#[derive(Debug, Clone)]
pub struct RuntimeToolPolicy {
    approval_policy: ApprovalPolicy,
    permission_profile: PermissionProfile,
    sandbox_mode: SandboxMode,
    permissions: PermissionConfig,
    project_trusted: bool,
}

impl RuntimeToolPolicy {
    /// Build runtime policy from a [`Config`]'s effective policy accessors.
    pub fn from_config(config: &Config) -> Self {
        Self {
            approval_policy: config.effective_approval_policy(),
            permission_profile: config.effective_permission_profile(),
            sandbox_mode: config.effective_sandbox_mode(),
            permissions: config.permissions.clone(),
            project_trusted: config.is_project_trusted(),
        }
    }

    /// Decide how a requested tool invocation should be handled.
    pub fn decide_tool(&self, tool_name: &str) -> ToolPolicyDecision {
        let tool_kind = ToolKind::for_name(tool_name);
        if let Some(outcome) = super::permissions::tool_outcome(&self.permissions, tool_name) {
            return ToolPolicyDecision::new(outcome, DecisionReason::MutatingTool, tool_kind);
        }
        let (outcome, reason) = super::base::decide(
            self.project_trusted,
            self.approval_policy,
            self.permission_profile,
            tool_kind,
        );
        ToolPolicyDecision::new(outcome, reason, tool_kind)
    }

    pub(crate) fn command_rule(&self, command: &str) -> Option<super::ToolPolicyOutcome> {
        super::command_rule::outcome(&self.permissions, command)
    }

    pub(crate) fn approval_policy(&self) -> ApprovalPolicy {
        self.approval_policy
    }

    pub(crate) fn sandbox_mode(&self) -> SandboxMode {
        self.sandbox_mode
    }
}
