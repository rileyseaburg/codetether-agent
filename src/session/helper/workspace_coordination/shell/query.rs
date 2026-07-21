//! Universal command metadata queries that cannot execute normal work.

pub(super) fn help_or_version(command: &str) -> bool {
    command
        .split_whitespace()
        .skip(1)
        .any(|word| matches!(word, "--help" | "--version" | "-V"))
}
