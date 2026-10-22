//! Tool-contract and shell-literal regression coverage; no builds are spawned.
use super::*;
use serde_json::json;

#[tokio::test]
async fn plan_is_explicit_and_never_executes_or_grants_approval() {
    let result = RustBuildPlanTool
        .execute(json!({
            "action": "check", "cwd": env!("CARGO_MANIFEST_DIR")
        }))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    let value: Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(value["executed"], false);
    assert_eq!(value["argv"][0], "rust");
    assert_eq!(value["argv"].as_array().unwrap().last().unwrap(), "check");
    assert!(value["exec_command"].get("approval_id").is_none());
    assert!(value["exec_command"].get("justification").is_none());
    assert!(!codetether_runtime_policy::command::is_read_only_command(
        value["exec_command"]["cmd"].as_str().unwrap()
    ));
    assert!(codetether_tool_core::readonly::is_read_only(
        "rust_build_plan"
    ));
}

#[tokio::test]
async fn refuses_injected_authority_and_unbounded_options() {
    for extra in [
        json!({"approval_id": "fake"}),
        json!({"action": "enable"}),
        json!({"toolchain": "stable;echo bad"}),
        json!({"cwd": "."}),
        json!({"args": ["--release"]}),
    ] {
        let mut input = json!({"action": "build", "cwd": env!("CARGO_MANIFEST_DIR")});
        input
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        let result = RustBuildPlanTool.execute(input).await.unwrap();
        assert!(!result.success, "{}", result.output);
    }
}

#[test]
fn shell_quoting_preserves_literal_arguments() {
    assert_eq!(output::quote("a'b", false), "'a'\\''b'");
    assert_eq!(output::quote("a'b", true), "'a''b'");
    #[cfg(unix)]
    {
        let value = "a'b $(echo injected); *\n next";
        let result = std::process::Command::new("/bin/sh")
            .args(["-c", &format!("printf %s {}", output::quote(value, false))])
            .output()
            .unwrap();
        assert!(result.status.success());
        assert_eq!(String::from_utf8(result.stdout).unwrap(), value);
    }
}
