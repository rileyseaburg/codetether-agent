//! Configuration system
//!
//! Handles loading configuration from multiple sources:
//! - Global config (~/.config/codetether/config.toml)
//! - Project config (./codetether.toml or .codetether/config.toml)
//! - Environment variables (CODETETHER_*)

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default provider to use
    #[serde(default)]
    pub default_provider: Option<String>,

    /// Default model to use (provider/model format)
    #[serde(default)]
    pub default_model: Option<String>,

    /// Provider-specific configurations
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,

    /// Agent configurations
    #[serde(default)]
    pub agents: HashMap<String, AgentConfig>,

    /// Permission rules
    #[serde(default)]
    pub permissions: PermissionConfig,

    /// A2A worker settings
    #[serde(default)]
    pub a2a: A2aConfig,

    /// UI/TUI settings
    #[serde(default)]
    pub ui: UiConfig,

    /// Session settings
    #[serde(default)]
    pub session: SessionConfig,

    /// Telemetry and crash reporting settings
    #[serde(default)]
    pub telemetry: TelemetryConfig,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    /// API key (can also be set via env var)
    pub api_key: Option<String>,

    /// Base URL override
    pub base_url: Option<String>,

    /// Custom headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Organization ID (for OpenAI)
    pub organization: Option<String>,
}

impl std::fmt::Debug for ProviderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderConfig")
            .field("api_key", &self.api_key.as_ref().map(|_| "<REDACTED>"))
            .field("api_key_len", &self.api_key.as_ref().map(|k| k.len()))
            .field("base_url", &self.base_url)
            .field("organization", &self.organization)
            .field("headers_count", &self.headers.len())
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent name
    pub name: String,

    /// Description
    #[serde(default)]
    pub description: Option<String>,

    /// Model override for this agent
    #[serde(default)]
    pub model: Option<String>,

    /// System prompt override
    #[serde(default)]
    pub prompt: Option<String>,

    /// Temperature setting
    #[serde(default)]
    pub temperature: Option<f32>,

    /// Top-p setting
    #[serde(default)]
    pub top_p: Option<f32>,

    /// Custom permissions for this agent
    #[serde(default)]
    pub permissions: HashMap<String, PermissionAction>,

    /// Whether this agent is disabled
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionConfig {
    /// Default permission rules
    #[serde(default)]
    pub rules: HashMap<String, PermissionAction>,

    /// Tool-specific permissions
    #[serde(default)]
    pub tools: HashMap<String, PermissionAction>,

    /// Path-specific permissions
    #[serde(default)]
    pub paths: HashMap<String, PermissionAction>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

impl Default for PermissionAction {
    fn default() -> Self {
        Self::Ask
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct A2aConfig {
    /// Default A2A server URL
    pub server_url: Option<String>,

    /// Worker name
    pub worker_name: Option<String>,

    /// Auto-approve policy
    #[serde(default)]
    pub auto_approve: AutoApprovePolicy,

    /// Codebases to register
    #[serde(default)]
    pub codebases: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AutoApprovePolicy {
    All,
    #[default]
    Safe,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme name ("default", "dark", "light", "solarized-dark", "solarized-light", or custom)
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Show line numbers in code
    #[serde(default = "default_true")]
    pub line_numbers: bool,

    /// Enable mouse support
    #[serde(default = "default_true")]
    pub mouse: bool,

    /// Custom theme configuration (overrides preset themes)
    #[serde(default)]
    pub custom_theme: Option<crate::tui::theme::Theme>,

    /// Enable theme hot-reloading (apply changes without restart)
    #[serde(default = "default_false")]
    pub hot_reload: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            line_numbers: true,
            mouse: true,
            custom_theme: None,
            hot_reload: false,
        }
    }
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Auto-compact sessions when they get too long
    #[serde(default = "default_true")]
    pub auto_compact: bool,

    /// Maximum context tokens before compaction
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    /// Enable session persistence
    #[serde(default = "default_true")]
    pub persist: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            auto_compact: true,
            max_tokens: default_max_tokens(),
            persist: true,
        }
    }
}

fn default_max_tokens() -> usize {
    100_000
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryConfig {
    /// Opt-in crash reporting. Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_reporting: Option<bool>,

    /// Whether we have already prompted the user about crash reporting consent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_reporting_prompted: Option<bool>,

    /// Endpoint for crash report ingestion.
    /// Defaults to the CodeTether telemetry endpoint when unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_report_endpoint: Option<String>,
}

impl TelemetryConfig {
    pub fn crash_reporting_enabled(&self) -> bool {
        self.crash_reporting.unwrap_or(false)
    }

    pub fn crash_reporting_prompted(&self) -> bool {
        self.crash_reporting_prompted.unwrap_or(false)
    }

    pub fn crash_report_endpoint(&self) -> String {
        self.crash_report_endpoint
            .clone()
            .unwrap_or_else(default_crash_report_endpoint)
    }
}

fn default_crash_report_endpoint() -> String {
    "https://telemetry.codetether.ai/v1/crash-reports".to_string()
}

impl Config {
    /// Load configuration from all sources (global, project, env)
    pub async fn load() -> Result<Self> {
        let mut config = Self::default();

        // Load global config
        if let Some(global_path) = Self::global_config_path() {
            if global_path.exists() {
                let content = fs::read_to_string(&global_path).await?;
                let global: Config = toml::from_str(&content)?;
                config = config.merge(global);
            }
        }

        // Load project config
        for name in ["codetether.toml", ".codetether/config.toml"] {
            let path = PathBuf::from(name);
            if path.exists() {
                let content = fs::read_to_string(&path).await?;
                let project: Config = toml::from_str(&content)?;
                config = config.merge(project);
            }
        }

        // Apply environment overrides
        config.apply_env();

        Ok(config)
    }

    /// Get the global config directory path
    pub fn global_config_path() -> Option<PathBuf> {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Get the data directory path
    pub fn data_dir() -> Option<PathBuf> {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .map(|dirs| dirs.data_dir().to_path_buf())
    }

    /// Initialize default configuration file
    pub async fn init_default() -> Result<()> {
        if let Some(path) = Self::global_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let default = Self::default();
            let content = toml::to_string_pretty(&default)?;
            fs::write(&path, content).await?;
            tracing::info!("Created config at {:?}", path);
        }
        Ok(())
    }

    /// Set a configuration value
    pub async fn set(key: &str, value: &str) -> Result<()> {
        let mut config = Self::load().await?;

        // Parse key path and set value
        match key {
            "default_provider" => config.default_provider = Some(value.to_string()),
            "default_model" => config.default_model = Some(value.to_string()),
            "a2a.server_url" => config.a2a.server_url = Some(value.to_string()),
            "a2a.worker_name" => config.a2a.worker_name = Some(value.to_string()),
            "ui.theme" => config.ui.theme = value.to_string(),
            "telemetry.crash_reporting" => {
                config.telemetry.crash_reporting = Some(parse_bool(value)?)
            }
            "telemetry.crash_reporting_prompted" => {
                config.telemetry.crash_reporting_prompted = Some(parse_bool(value)?)
            }
            "telemetry.crash_report_endpoint" => {
                config.telemetry.crash_report_endpoint = Some(value.to_string())
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }

        // Save to global config
        if let Some(path) = Self::global_config_path() {
            let content = toml::to_string_pretty(&config)?;
            fs::write(&path, content).await?;
        }

        Ok(())
    }

    /// Merge two configs (other takes precedence)
    fn merge(mut self, other: Self) -> Self {
        if other.default_provider.is_some() {
            self.default_provider = other.default_provider;
        }
        if other.default_model.is_some() {
            self.default_model = other.default_model;
        }
        self.providers.extend(other.providers);
        self.agents.extend(other.agents);
        self.permissions.rules.extend(other.permissions.rules);
        self.permissions.tools.extend(other.permissions.tools);
        self.permissions.paths.extend(other.permissions.paths);
        if other.a2a.server_url.is_some() {
            self.a2a = other.a2a;
        }
        if other.telemetry.crash_reporting.is_some() {
            self.telemetry.crash_reporting = other.telemetry.crash_reporting;
        }
        if other.telemetry.crash_reporting_prompted.is_some() {
            self.telemetry.crash_reporting_prompted = other.telemetry.crash_reporting_prompted;
        }
        if other.telemetry.crash_report_endpoint.is_some() {
            self.telemetry.crash_report_endpoint = other.telemetry.crash_report_endpoint;
        }
        self
    }

    /// Load theme based on configuration
    pub fn load_theme(&self) -> crate::tui::theme::Theme {
        // Use custom theme if provided
        if let Some(custom) = &self.ui.custom_theme {
            return custom.clone();
        }

        // Use preset theme
        match self.ui.theme.as_str() {
            "dark" | "default" => crate::tui::theme::Theme::dark(),
            "light" => crate::tui::theme::Theme::light(),
            "solarized-dark" => crate::tui::theme::Theme::solarized_dark(),
            "solarized-light" => crate::tui::theme::Theme::solarized_light(),
            _ => {
                // Log warning and fallback to dark theme
                tracing::warn!(theme = %self.ui.theme, "Unknown theme name, falling back to dark");
                crate::tui::theme::Theme::dark()
            }
        }
    }

    /// Apply environment variable overrides
    fn apply_env(&mut self) {
        if let Ok(val) = std::env::var("CODETETHER_DEFAULT_MODEL") {
            self.default_model = Some(val);
        }
        if let Ok(val) = std::env::var("CODETETHER_DEFAULT_PROVIDER") {
            self.default_provider = Some(val);
        }
        if let Ok(val) = std::env::var("OPENAI_API_KEY") {
            self.providers
                .entry("openai".to_string())
                .or_default()
                .api_key = Some(val);
        }
        if let Ok(val) = std::env::var("ANTHROPIC_API_KEY") {
            self.providers
                .entry("anthropic".to_string())
                .or_default()
                .api_key = Some(val);
        }
        if let Ok(val) = std::env::var("GOOGLE_API_KEY") {
            self.providers
                .entry("google".to_string())
                .or_default()
                .api_key = Some(val);
        }
        if let Ok(val) = std::env::var("CODETETHER_A2A_SERVER") {
            self.a2a.server_url = Some(val);
        }
        if let Ok(val) = std::env::var("CODETETHER_CRASH_REPORTING") {
            match parse_bool(&val) {
                Ok(enabled) => self.telemetry.crash_reporting = Some(enabled),
                Err(_) => tracing::warn!(
                    value = %val,
                    "Invalid CODETETHER_CRASH_REPORTING value; expected true/false"
                ),
            }
        }
        if let Ok(val) = std::env::var("CODETETHER_CRASH_REPORT_ENDPOINT") {
            self.telemetry.crash_report_endpoint = Some(val);
        }
    }
}

fn parse_bool(value: &str) -> Result<bool> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => anyhow::bail!("Invalid boolean value: {}", value),
    }
}
