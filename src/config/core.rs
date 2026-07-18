use crate::config::guardrails::CostGuardrails;
use crate::config::{
    A2aConfig, AccessMode, AgentsConfig, ApprovalPolicy, LspSettings, PermissionConfig,
    PermissionProfileConfig, PolicyRequirements, ProjectTrustLevel, ProviderConfig, SandboxMode,
    SessionConfig, TelemetryConfig, UiConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_provider: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
    #[serde(default)]
    pub access_mode: Option<AccessMode>,
    #[serde(default)]
    pub sandbox_mode: Option<SandboxMode>,
    #[serde(default)]
    pub approval_policy: Option<ApprovalPolicy>,
    #[serde(default)]
    pub trust_level: Option<ProjectTrustLevel>,
    #[serde(default)]
    pub permission_profile: Option<PermissionProfileConfig>,
    #[serde(default)]
    pub requirements: PolicyRequirements,
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    #[serde(default)]
    pub agents: AgentsConfig,
    #[serde(default)]
    pub permissions: PermissionConfig,
    #[serde(default)]
    pub a2a: A2aConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub session: SessionConfig,
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    #[serde(default)]
    pub guardrails: CostGuardrails,
    #[serde(default)]
    pub lsp: LspSettings,
    #[serde(default)]
    pub rlm: crate::rlm::RlmConfig,
}
