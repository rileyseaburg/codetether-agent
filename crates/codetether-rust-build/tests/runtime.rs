use codetether_rust_build::{RustAction as A, RustArgs, run};

#[tokio::test]
async fn missing_manifest_fails_before_any_tool_is_invoked() {
    let directory = tempfile::tempdir().unwrap();
    for action in [A::Build, A::Run, A::Test, A::Check] {
        let input = RustArgs {
            action,
            cwd: directory.path().into(),
            toolchain: "nonexistent-toolchain".into(),
            ..Default::default()
        };
        let error = run(input).await.unwrap_err().to_string();
        assert!(error.contains("Cargo.toml required"), "{error}");
    }
}

#[tokio::test]
async fn invalid_input_fails_before_any_tool_is_invoked() {
    let input = RustArgs {
        toolchain: "--help".into(),
        ..Default::default()
    };
    assert!(
        run(input)
            .await
            .unwrap_err()
            .to_string()
            .contains("invalid toolchain")
    );
}
