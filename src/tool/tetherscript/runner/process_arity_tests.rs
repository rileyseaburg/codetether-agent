//! Arity regression test for the `process_run` authority bridge.
//!
//! `process_run` must expose exactly one `Result` layer on every execution path.
//! It previously double-wrapped under the `tetherscript_plugin` tool (because the
//! authority returned an already-wrapped `Value::Result`, which the interpreter
//! then lifted again), so a plugin needed `??` there but `?` under
//! `tetherscript run`. No single plugin source was correct on both paths, and
//! indexing the inner `Result` failed with "cannot index result with str".

use crate::tool::tetherscript::TetherScriptPluginTool;
use crate::tool::{Tool, ToolResult};
use serde_json::json;

/// Plugin source that indexes the result map after a single `?`.
const SINGLE_QUESTION: &str = r#"
fn probe() {
    let res = process_run("echo", ["arity-ok"], nil, 5000)?
    return Ok(str(res["stdout"]))
}
"#;

async fn run_probe(source: &str) -> ToolResult {
    let _lock = crate::approval::test_env::lock_env();
    let dir = tempfile::tempdir().expect("tempdir");
    let _env = crate::approval::test_env::ScopedEnv::data_dir_with_access(
        dir.path(),
        crate::config::AccessMode::Full,
    );
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let path = dir.path().join("arity_probe.tether");
    std::fs::write(&path, source).expect("write plugin");
    // `new()` pins the workspace root, and a tempdir path escapes it by design,
    // so scope the tool to the tempdir instead of weakening that guard.
    TetherScriptPluginTool::with_root(dir.path().to_path_buf())
        .execute(json!({
            "path": path.to_string_lossy(),
            "hook": "probe",
            "args": [],
            "grant_process": true,
        }))
        .await
        .expect("plugin execution")
}

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn process_run_exposes_exactly_one_result_layer() {
    crate::tool::tetherscript::require_sandbox!();
    let result = run_probe(SINGLE_QUESTION).await;
    assert!(
        result.success,
        "single `?` must suffice on the plugin path: {}",
        result.output
    );
    assert!(
        result.output.contains("arity-ok"),
        "expected subprocess stdout, got {}",
        result.output
    );
    assert!(
        !result.output.contains("cannot index result"),
        "double-wrapped Result regressed: {}",
        result.output
    );
}
