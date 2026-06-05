use crate::config::Config;
use crate::config::bool_parse::parse_bool;
use anyhow::Result;
use tokio::fs;

impl Config {
    /// Initialize default configuration file.
    pub async fn init_default() -> Result<()> {
        if let Some(path) = Self::global_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let content = toml::to_string_pretty(&Self::default())?;
            fs::write(&path, content).await?;
            tracing::info!(path = ?path, "Created config");
        }
        Ok(())
    }

    /// Set a configuration value in the global config file.
    pub async fn set(key: &str, value: &str) -> Result<()> {
        let mut config = Self::load().await?;
        config.set_value(key, value)?;
        if let Some(path) = Self::global_config_path() {
            fs::write(&path, toml::to_string_pretty(&config)?).await?;
        }
        Ok(())
    }

    fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default_provider" => self.default_provider = Some(value.to_string()),
            "default_model" => self.default_model = Some(value.to_string()),
            "a2a.server_url" => self.a2a.server_url = Some(value.to_string()),
            "a2a.worker_name" => self.a2a.worker_name = Some(value.to_string()),
            "ui.theme" => self.ui.theme = value.to_string(),
            "telemetry.crash_reporting" => {
                self.telemetry.crash_reporting = Some(parse_bool(value)?)
            }
            "telemetry.crash_reporting_prompted" => {
                self.telemetry.crash_reporting_prompted = Some(parse_bool(value)?)
            }
            "telemetry.crash_report_endpoint" => {
                self.telemetry.crash_report_endpoint = Some(value.to_string())
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        Ok(())
    }
}
