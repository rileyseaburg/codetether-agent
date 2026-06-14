use std::path::{Path, PathBuf};

const ENV_RUN_KNOWLEDGE_SNAPSHOT: &str = "CODETETHER_RUN_KNOWLEDGE_SNAPSHOT";

pub(super) async fn refresh(workspace_dir: &Path) -> Option<PathBuf> {
    if !enabled() {
        tracing::debug!(
            env = ENV_RUN_KNOWLEDGE_SNAPSHOT,
            "Skipping workspace knowledge snapshot for run"
        );
        return None;
    }

    match crate::indexer::refresh_workspace_knowledge_snapshot(workspace_dir).await {
        Ok(path) => {
            tracing::info!(
                workspace = %workspace_dir.display(),
                output = %path.display(),
                "Refreshed workspace knowledge snapshot for run"
            );
            Some(path)
        }
        Err(error) => {
            tracing::warn!(
                workspace = %workspace_dir.display(),
                error = %error,
                "Failed to refresh workspace knowledge snapshot"
            );
            None
        }
    }
}

fn enabled() -> bool {
    std::env::var(ENV_RUN_KNOWLEDGE_SNAPSHOT).is_ok_and(|value| enabled_value(&value))
}

fn enabled_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
mod tests {
    use super::enabled_value;

    #[test]
    fn parses_enabled_values() {
        assert!(enabled_value("1"));
        assert!(enabled_value("true"));
        assert!(enabled_value("YES"));
        assert!(enabled_value("on"));
        assert!(!enabled_value("false"));
    }
}
