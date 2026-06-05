use crate::config::guardrails::CostGuardrails;
use crate::config::{
    A2aConfig, AgentConfig, LspSettings, PermissionConfig, ProviderConfig, SessionConfig,
    TelemetryConfig, UiConfig,
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
    pub providers: HashMap<String, ProviderConfig>,
    #[serde(default)]
    pub agents: HashMap<String, AgentConfig>,
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
