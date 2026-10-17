use crate::config::Config;
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::fs;

impl Config {
    /// Load configuration from all sources.
    pub async fn load() -> Result<Self> {
        let global = Self::read_global_config();
        let project = read_project_configs();
        let (global_result, project_results) = tokio::join!(global, project);
        let mut config = Self::default();
        if let Some((path, Some(content))) = global_result {
            config = config.merge(parse_config(&path, &content)?);
        }
        config = super::project_policy::apply_trust_store(config);
        for (path, maybe) in project_results {
            if let Some(content) = maybe {
                let overlay = parse_config(&path, &content)?;
                let overlay = super::project_policy::sanitize_project_overlay(&config, overlay);
                config = config.merge(overlay);
            }
        }
        config.apply_env();
        if let Some(mode) = super::access_mode_runtime::process_access_mode_override() {
            config.apply_access_mode_override(Some(mode));
        }
        config.normalize_legacy_defaults();
        Ok(config)
    }

    async fn read_global_config() -> Option<(PathBuf, Option<String>)> {
        Some(read_opt(Self::global_config_path()?).await)
    }
}

async fn read_project_configs() -> Vec<(PathBuf, Option<String>)> {
    let paths = [
        PathBuf::from("codetether.toml"),
        PathBuf::from(".codetether/config.toml"),
    ];
    futures::future::join_all(paths.into_iter().map(read_opt)).await
}

async fn read_opt(path: PathBuf) -> (PathBuf, Option<String>) {
    let content = fs::read_to_string(&path).await.ok();
    (path, content)
}

fn parse_config(path: &PathBuf, content: &str) -> Result<Config> {
    toml::from_str::<Config>(content).with_context(|| format!("failed to parse {}", path.display()))
}
