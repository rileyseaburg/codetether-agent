use crate::config::guardrails::CostGuardrails;
use crate::config::{
    A2aConfig, Config, LspSettings, PermissionConfig, SessionConfig, TelemetryConfig, UiConfig,
};
use std::collections::HashMap;

impl Default for Config {
    fn default() -> Self {
        Self {
            default_provider: Some("minimax".to_string()),
            default_model: Some("minimax/MiniMax-M3".to_string()),
            providers: HashMap::new(),
            agents: HashMap::new(),
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
