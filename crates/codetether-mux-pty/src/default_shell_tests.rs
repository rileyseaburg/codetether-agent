//! Login-shell command regression tests.

#[cfg(unix)]
#[test]
fn honors_configured_unix_shell() {
    assert_eq!(super::unix_command(Some("/bin/zsh")), "exec '/bin/zsh' -l");
}

#[cfg(unix)]
#[test]
fn quotes_shell_paths_without_injection() {
    assert_eq!(
        super::unix_command(Some("/tmp/a'b")),
        "exec '/tmp/a'\"'\"'b' -l"
    );
}
