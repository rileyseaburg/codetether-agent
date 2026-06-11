use super::deny_protected_writes;

#[test]
fn denies_redirect_to_git_metadata() {
    let args = vec!["-c".to_string(), "echo x > .git/config".to_string()];
    assert!(deny_protected_writes(&args).is_err());
}

#[test]
fn denies_touching_codex_metadata() {
    let args = vec!["-c".to_string(), "touch .codex/settings".to_string()];
    assert!(deny_protected_writes(&args).is_err());
}

#[test]
fn allows_metadata_reads() {
    let args = vec!["-c".to_string(), "cat .git/config".to_string()];
    assert!(deny_protected_writes(&args).is_ok());
}
