//! Migration guard for plugins written against the double-wrapped `process_run`.
//!
//! `bash_guard.tether` and `bedrock_model_probe.tether` used `??` because the
//! authority returned `Result<Result<map>>`. The arity fix removes the extra
//! layer, so `??` is now a hard error rather than a tolerated redundancy.
//!
//! This test pins that the break is *loud*. An earlier, permissive version of
//! this test accepted either outcome and therefore missed a real regression in
//! two checked-in plugins; the failure surfaced only in the `bash_guard` suite.

#[path = "process_migration_test_run.rs"]
mod run;

use serde_json::json;

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
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn single_unwrap_is_the_supported_arity() {
    crate::tool::tetherscript::require_sandbox!();
    let result = run::hook(SINGLE, "probe", json!(["echo migrated"])).await;
    assert!(result.success, "one `?` must work: {}", result.output);
    assert!(result.output.contains("migrated"), "got {}", result.output);
}

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn stale_double_unwrap_fails_loudly_instead_of_silently() {
    crate::tool::tetherscript::require_sandbox!();
    let result = run::hook(DOUBLE, "probe", json!(["echo stale"])).await;
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
