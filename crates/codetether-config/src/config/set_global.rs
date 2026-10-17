//! Global config file helpers for `config --set`.

use crate::config::Config;
use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub(super) async fn load(path: &Path) -> Result<Config> {
    match fs::read_to_string(path).await {
        Ok(content) => Ok(toml::from_str::<Config>(&content)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(error) => Err(error.into()),
    }
}

pub(super) async fn write(path: &Path, config: &Config) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(fs::write(path, toml::to_string_pretty(config)?).await?)
}
