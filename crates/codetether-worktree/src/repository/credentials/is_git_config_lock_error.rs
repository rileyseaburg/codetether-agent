//! Credential configuration operation: is git config lock error.

pub(super) fn is_git_config_lock_error(message: &str) -> bool {
    message.contains("could not lock config file") && message.contains("File exists")
}
