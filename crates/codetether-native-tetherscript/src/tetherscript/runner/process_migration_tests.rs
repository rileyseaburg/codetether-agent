//! Migration guard for plugins written against the double-wrapped `process_run`.
//!
//! `bash_guard.tether` and `bedrock_model_probe.tether` used `??` because the
//! authority returned `Result<Result<map>>`. The arity fix removes the extra
//! layer, so `??` is now a hard error rather than a tolerated redundancy.
//!
//! This test pins that the break is *loud*. An earlier, permissive version of
//! this test accepted either outcome and therefore missed a real regression in
//! two checked-in plugins; the failure surfaced only in the `bash_guard` suite.

use crate::tool::tetherscript::TetherScriptPluginTool;
use crate::tool::{Tool, ToolResult};
use serde_json::json;

async fn run_hook(source: &str, hook: &str, args: serde_json::Value) -> ToolResult {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("probe.tether");
    std::fs::write(&path, source).expect("write plugin");
    TetherScriptPluginTool::with_root(dir.path().to_path_buf())
        .execute(json!({
            "path": path.to_string_lossy(),
            "hook": hook,
            "args": args,
        }))
        .await
        .expect("plugin execution")
}

/// Correct arity after the fix: one `?` yields the result map.
const SINGLE: &str = r#"
fn probe(command) {
    let res = process_run("bash", ["-c", command], nil, 5000)?
    return Ok(str(res["stdout"]))
}
"#;

/// Stale arity: `??` unwraps past the map.
const DOUBLE: &str = r#"
fn probe(command) {
    let res = process_run("bash", ["-c", command], nil, 5000)??
    return Ok(str(res["stdout"]))
}
"#;

#[tokio::test]
async fn single_unwrap_is_the_supported_arity() {
    let result = run_hook(SINGLE, "probe", json!(["echo migrated"])).await;
    assert!(result.success, "one `?` must work: {}", result.output);
    assert!(result.output.contains("migrated"), "got {}", result.output);
}

#[tokio::test]
async fn stale_double_unwrap_fails_loudly_instead_of_silently() {
    let result = run_hook(DOUBLE, "probe", json!(["echo stale"])).await;
    assert!(
        !result.success,
        "`??` must not silently succeed after the arity fix: {}",
        result.output
    );
    assert!(
        result.output.contains("expected Result"),
        "error must name the arity problem so authors can migrate, got {}",
        result.output
    );
}
