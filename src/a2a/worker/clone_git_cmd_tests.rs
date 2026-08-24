//! A2A Git requires explicit network authority and remains sandboxed.

#[tokio::test]
async fn worker_git_fails_closed_without_process_network_policy() {
    let _lock = crate::approval::test_env::lock_env();
    let _network = crate::tool::network_access::test_env::Network::set("0");
    let error = super::run_git_command_at(None, vec!["--version".into()])
        .await
        .unwrap_err();
    assert!(error.to_string().contains("network access is disabled"));
}

#[tokio::test]
async fn authorized_worker_git_runs_through_sandbox() {
    let _lock = crate::approval::test_env::lock_env();
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let output = super::run_git_command_at(None, vec!["--version".into()])
        .await
        .expect("sandboxed git");
    assert!(output.starts_with("git version"), "{output}");
}
