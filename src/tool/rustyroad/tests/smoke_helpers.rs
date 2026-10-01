//! Nonsecret fixture setup and concrete installed-backend output assertions.

use crate::tool::ToolResult;
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn project() -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    let fixture = include_str!("../../../../tests/fixtures/rustyroad_config.toml");
    for (file, name) in [
        ("rustyroad.toml", "dev_fixture"),
        ("rustyroad.test.toml", "test_fixture"),
    ] {
        std::fs::write(
            project.path().join(file),
            fixture.replace("rustyroad_fixture", name),
        )
        .unwrap();
    }
    project
}

pub(super) fn assert_config(
    result: &ToolResult,
    project: &Path,
    environment: &str,
    database: &str,
) {
    assert!(result.success, "{}", result.output);
    let config: Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(config["environment"], environment);
    assert_eq!(config["database"]["name"], database);
    assert_eq!(
        result.metadata["rustyroad_project"],
        json!(project.canonicalize().unwrap())
    );
    assert_eq!(result.metadata["rustyroad_environment"], environment);
    assert!(
        result.metadata["rustyroad_version"]
            .as_str()
            .is_some_and(|version| !version.is_empty())
    );
}
