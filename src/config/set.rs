use crate::config::Config;
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
        let Some(path) = Self::global_config_path() else {
            return Ok(());
        };
        let mut config = super::set_global::load(&path).await?;
        config.set_value(key, value)?;
        super::set_global::write(&path, &config).await?;
        Ok(())
    }
}
