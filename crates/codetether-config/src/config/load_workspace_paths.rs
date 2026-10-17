use crate::config::Config;
use crate::config::path;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

pub(super) async fn read_global_config() -> Option<(PathBuf, Option<String>)> {
    Some(read_opt(Config::global_config_path()?).await)
}

pub(super) async fn read_project_configs(root: &Path) -> Vec<(PathBuf, Option<String>)> {
    let paths = [
        root.join("codetether.toml"),
        root.join(".codetether/config.toml"),
    ];
    futures::future::join_all(paths.into_iter().map(read_opt)).await
}

pub(super) async fn read_opt(path: PathBuf) -> (PathBuf, Option<String>) {
    let content = fs::read_to_string(&path).await.ok();
    (path, content)
}

pub(super) fn parse_config(path: &PathBuf, content: &str) -> Result<Config> {
    toml::from_str::<Config>(content).with_context(|| format!("failed to parse {}", path.display()))
}

pub(super) fn workspace_root(path: &Path) -> PathBuf {
    path::detect_workspace_root(path).unwrap_or_else(|| path.to_path_buf())
}
