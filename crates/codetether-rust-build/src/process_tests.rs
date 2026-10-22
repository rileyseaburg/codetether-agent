use crate::{process, validate::version_matches};

#[test]
fn exact_version_is_required() {
    assert!(version_matches("rustc 1.99.0 (hash date)\n"));
    for version in [
        "rustc 1.99.1",
        "rustc 1.99.0-nightly",
        "rustc 1.100.0",
        "1.99.0",
    ] {
        assert!(!version_matches(version));
    }
}

#[test]
#[ignore = "subprocess fixture, invoked by the process test"]
fn child_success() {
    assert_eq!(std::env::var("RUSTUP_AUTO_INSTALL").unwrap(), "0");
}

#[test]
#[ignore = "subprocess fixture, invoked by the process test"]
fn child_failure() {
    std::process::exit(23);
}

#[tokio::test]
async fn mock_child_success_capture_and_failure_propagate() {
    let executable = std::env::current_exe().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let argv = vec![
        "--ignored".into(),
        "--exact".into(),
        "process_tests::child_success".into(),
    ];
    process::execute(executable.to_str().unwrap(), &argv, directory.path())
        .await
        .unwrap();
    let output = process::capture(executable.to_str().unwrap(), &argv, directory.path())
        .await
        .unwrap();
    assert!(output.contains("1 passed"));
    let argv = vec![
        "--ignored".into(),
        "--exact".into(),
        "process_tests::child_failure".into(),
    ];
    let error = process::execute(executable.to_str().unwrap(), &argv, directory.path())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("23"));
}

#[tokio::test]
async fn missing_executable_is_an_error() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("absent-program");
    assert!(
        process::execute(missing.to_str().unwrap(), &[], directory.path())
            .await
            .is_err()
    );
}
