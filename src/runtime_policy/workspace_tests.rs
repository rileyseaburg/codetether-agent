use super::evaluate_tool_invocation;
use crate::approval::test_env::lock_env;
use crate::config::ProjectTrustStore;
use serde_json::json;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn policy_loads_config_for_tool_cwd_workspace() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = EnvGuard::data_dir(data.path());
    let workspace = tempfile::tempdir().expect("workspace");
    std::fs::create_dir(workspace.path().join(".git")).expect("git dir");
    std::fs::write(
        workspace.path().join("codetether.toml"),
        r#"approval_policy = "never"
trust_level = "trusted"
"#,
    )
    .expect("config");
    ProjectTrustStore::for_workspace(workspace.path())
        .expect("trust store")
        .trust()
        .expect("trust workspace");
    let args = json!({
        "command": "cargo test",
        "cwd": workspace.path().display().to_string()
    });

    assert!(evaluate_tool_invocation("bash", &args).await.is_none());
}
