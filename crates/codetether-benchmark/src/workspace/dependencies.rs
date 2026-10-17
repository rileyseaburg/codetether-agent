//! Add optional project dependencies inferred from feature and story text.
use std::{path::Path, process::Command};
pub(super) fn add_dependencies(workspace: &Path, feature_lower: &str, desc_text: &str) {
    if feature_lower.contains("api")
        || feature_lower.contains("rest")
        || desc_text.contains("axum")
        || desc_text.contains("endpoint")
    {
        let cargo_add_args = vec!["add", "axum", "tokio", "--features", "tokio/full"];
        let _ = Command::new("cargo")
            .args(&cargo_add_args)
            .current_dir(workspace)
            .output();
        // Also add serde_json for JSON responses
        let _ = Command::new("cargo")
            .args(["add", "serde_json"])
            .current_dir(workspace)
            .output();
    }

    if desc_text.contains("clap") || feature_lower.contains("cli") {
        let _ = Command::new("cargo")
            .args(["add", "clap", "--features", "derive"])
            .current_dir(workspace)
            .output();
    }

    if desc_text.contains("csv") {
        let _ = Command::new("cargo")
            .args(["add", "csv"])
            .current_dir(workspace)
            .output();
    }
}
