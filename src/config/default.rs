use crate::config::guardrails::CostGuardrails;
use crate::config::{
    A2aConfig, AgentsConfig, Config, LspSettings, PermissionConfig, SessionConfig, TelemetryConfig,
    UiConfig,
};
use std::collections::HashMap;

impl Default for Config {
    fn default() -> Self {
        Self {
            default_provider: Some("minimax".to_string()),
            default_model: Some("minimax/MiniMax-M3".to_string()),
            access_mode: None,
            sandbox_mode: None,
            approval_policy: None,
            trust_level: None,
            permission_profile: None,
            requirements: Default::default(),
            providers: HashMap::new(),
            agents: AgentsConfig::default(),
            permissions: PermissionConfig::default(),
            a2a: A2aConfig::default(),
            ui: UiConfig::default(),
            session: SessionConfig::default(),
            telemetry: TelemetryConfig::default(),
            guardrails: CostGuardrails::default(),
            lsp: LspSettings::default(),
            rlm: crate::rlm::RlmConfig::default(),
        }
    }
}
