//! LSP rust server definitions and helpers.

pub(super) fn rust_timeout() -> u64 {
    120000
}

pub(super) fn rust_analyzer_command() -> String {
    if which::which("rust-analyzer").is_ok() {
        "rust-analyzer".to_string()
    } else {
        "rustup".to_string()
    }
}

pub(super) fn rust_analyzer_args() -> Vec<String> {
    if which::which("rust-analyzer").is_ok() {
        Vec::new()
    } else {
        vec![
            "run".to_string(),
            "stable".to_string(),
            "rust-analyzer".to_string(),
        ]
    }
}
