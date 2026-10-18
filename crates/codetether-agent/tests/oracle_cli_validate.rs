use std::process::Command;

#[test]
fn oracle_validate_golden_failed_unverified() {
    let temp = tempfile::tempdir().expect("tempdir");
    let src_path = temp.path().join("sample.rs");
    std::fs::write(&src_path, "fn main() {}\npub async fn analyze() {}\n")
        .expect("write source file");

    let golden_payload = r#"{"kind":"grep","file":"sample.rs","pattern":"async fn","matches":[{"line":2,"text":"pub async fn analyze() {}"}]}"#;
    let failed_payload = r#"{"kind":"grep","file":"sample.rs","pattern":"async fn","matches":[{"line":999,"text":"pub async fn analyze() {}"}]}"#;
    let semantic_payload =
        r#"{"kind":"semantic","file":"sample.rs","answer":"This module has async analysis"}"#;

    let golden = run_oracle_validate(&src_path, "Find all async functions", golden_payload);
    assert!(
        golden.status.success(),
        "golden stderr: {}",
        stderr(&golden)
    );
    let golden_json = parse_json_from_output(&golden.stdout).expect("golden JSON output");
    assert_eq!(golden_json["oracle"]["status"], "golden");

    let failed = run_oracle_validate(&src_path, "Find all async functions", failed_payload);
    assert!(
        failed.status.success(),
        "failed stderr: {}",
        stderr(&failed)
    );
    let failed_json = parse_json_from_output(&failed.stdout).expect("failed JSON output");
    assert_eq!(failed_json["oracle"]["status"], "failed");
    assert!(
        !failed_json["oracle"]["reason"]
            .as_str()
            .unwrap_or("")
            .is_empty()
    );
    assert!(
        !failed_json["oracle"]["diff"]
            .as_str()
            .unwrap_or("")
            .is_empty()
    );

    let unverified = run_oracle_validate(&src_path, "Explain this module", semantic_payload);
    assert!(
        unverified.status.success(),
        "unverified stderr: {}",
        stderr(&unverified)
    );
    let unverified_json =
        parse_json_from_output(&unverified.stdout).expect("unverified JSON output");
    assert_eq!(unverified_json["oracle"]["status"], "unverified");
}

fn run_oracle_validate(
    source_path: &std::path::Path,
    query: &str,
    payload: &str,
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "oracle",
            "validate",
            "--query",
            query,
            "--file",
            source_path.to_str().expect("path utf8"),
            "--payload",
            payload,
            "--json",
        ])
        .output()
        .expect("run codetether oracle validate")
}

fn stderr(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).to_string()
}

fn parse_json_from_output(stdout: &[u8]) -> Result<serde_json::Value, serde_json::Error> {
    let text = String::from_utf8_lossy(stdout);
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
        return Ok(v);
    }

    if let Some(start) = text.find('{') {
        return serde_json::from_str(&text[start..]);
    }

    serde_json::from_str(&text)
}
