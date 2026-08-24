//! Process-enabled plugin fixture for migration tests.

use crate::tool::tetherscript::TetherScriptPluginTool;
use crate::tool::{Tool, ToolResult};
use serde_json::{Value, json};

pub(super) async fn hook(source: &str, hook: &str, args: Value) -> ToolResult {
    let _lock = crate::approval::test_env::lock_env();
    let dir = tempfile::tempdir().expect("tempdir");
    let _env = crate::approval::test_env::ScopedEnv::data_dir_with_access(
        dir.path(),
        crate::config::AccessMode::Full,
    );
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let path = dir.path().join("probe.tether");
    std::fs::write(&path, source).expect("write plugin");
    TetherScriptPluginTool::with_root(dir.path().to_path_buf())
        .execute(json!({
            "path": path.to_string_lossy(), "hook": hook,
            "args": args, "grant_process": true,
        }))
        .await
        .expect("plugin execution")
}
