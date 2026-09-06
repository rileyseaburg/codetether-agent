//! Process-level regression: installer probes must bypass agent/telemetry startup.

#[test]
#[cfg(not(windows))]
fn windows_probe_does_not_initialize_agent_state() {
    let directory = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_codetether"))
        .current_dir(directory.path())
        .env("HOME", directory.path().join("unused-home"))
        .env("XDG_DATA_HOME", directory.path().join("unused-data"))
        .args(["windows", "ocr-status", "--require-ready"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "Linux must not claim Windows OCR readiness"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(serde_json::from_str::<serde_json::Value>(&stdout).is_ok());
    assert!(!stderr.contains("telemetry"), "{stderr}");
    assert!(!stderr.contains("Vault"), "{stderr}");
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
}
