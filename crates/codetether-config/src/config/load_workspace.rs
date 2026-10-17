use crate::config::{Config, ProjectTrustLevel};
use anyhow::Result;
use std::path::Path;

#[path = "load_workspace_paths.rs"]
mod paths;

impl Config {
    /// Load configuration as if the supplied path were the active workspace.
    pub async fn load_for_workspace(path: impl AsRef<Path>) -> Result<Self> {
        let root = paths::workspace_root(path.as_ref());
        let global = paths::read_global_config();
        let project = paths::read_project_configs(&root);
        let (global_result, project_results) = tokio::join!(global, project);
        let mut config = Self::default();
        if let Some((path, Some(content))) = global_result {
            config = config.merge(paths::parse_config(&path, &content)?);
        }
        config = apply_workspace_trust(config, &root);
        for (path, maybe) in project_results {
            if let Some(content) = maybe {
                let overlay = super::project_policy::sanitize_project_overlay(
                    &config,
                    paths::parse_config(&path, &content)?,
                );
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
}

fn apply_workspace_trust(mut config: Config, root: &Path) -> Config {
    let trusted = super::trust_store::ProjectTrustStore::for_workspace(root)
        .is_ok_and(|store| store.is_trusted());
    if !config.is_project_trusted() && trusted {
        config.trust_level = Some(ProjectTrustLevel::Trusted);
    }
    config
}
