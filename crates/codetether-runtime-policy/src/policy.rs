//! Config-backed runtime tool policy.

use super::{DecisionReason, ToolKind, ToolPolicyDecision};
use codetether_config::{ApprovalPolicy, Config, PermissionConfig, PermissionProfile, SandboxMode};

/// Reusable decision state derived from effective configuration.
///
/// # Examples
/// ```
/// use codetether_runtime_policy::{RuntimeToolPolicy, ToolPolicyOutcome};
/// let policy = RuntimeToolPolicy::from_config(&codetether_config::Config::default());
/// assert_eq!(policy.decide_tool("read").outcome, ToolPolicyOutcome::Allow);
/// ```
#[derive(Debug, Clone)]
pub struct RuntimeToolPolicy {
    /// Effective approval prompting policy.
    pub approval_policy: ApprovalPolicy,
    /// Effective permission boundary.
    pub permission_profile: PermissionProfile,
    /// Requested operating-system sandbox mode.
    pub sandbox_mode: SandboxMode,
    /// Explicit tool and command-prefix overrides.
    pub permissions: PermissionConfig,
    /// Whether the current project is trusted.
    pub project_trusted: bool,
}

impl RuntimeToolPolicy {
    /// Build runtime policy from a [`Config`]'s effective policy accessors.
    ///
    /// # Arguments
    /// * `config` — Configuration whose effective settings will be captured.
    /// # Returns
    /// A reusable policy snapshot.
    /// # Examples
    /// ```
    /// let policy = codetether_runtime_policy::RuntimeToolPolicy::from_config(&codetether_config::Config::default());
    /// assert!(matches!(policy.decide_tool("read").tool_kind, codetether_runtime_policy::ToolKind::ReadOnly));
    /// ```
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
    ///
    /// # Arguments
    /// * `tool_name` — Tool identifier to classify.
    /// # Returns
    /// The explicit override or default policy decision.
    /// # Examples
    /// ```
    /// use codetether_runtime_policy::{RuntimeToolPolicy, ToolPolicyOutcome};
    /// let policy = RuntimeToolPolicy::from_config(&codetether_config::Config::default());
    /// assert_eq!(policy.decide_tool("read").outcome, ToolPolicyOutcome::Allow);
    /// ```
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

    /// Resolve the longest configured command-prefix rule, if any.
    ///
    /// # Arguments
    /// * `command` — Shell command text.
    /// # Returns
    /// The longest matching configured rule, or `None`.
    /// # Examples
    /// ```
    /// let policy = codetether_runtime_policy::RuntimeToolPolicy::from_config(&codetether_config::Config::default());
    /// assert_eq!(policy.command_rule("unconfigured-command"), None);
    /// ```
    pub fn command_rule(&self, command: &str) -> Option<super::ToolPolicyOutcome> {
        super::command_rule::outcome(&self.permissions, command)
    }

    /// Return the effective approval policy.
    pub fn approval_policy(&self) -> ApprovalPolicy {
        self.approval_policy
    }

    /// Return the effective sandbox mode.
    pub fn sandbox_mode(&self) -> SandboxMode {
        self.sandbox_mode
    }
}
