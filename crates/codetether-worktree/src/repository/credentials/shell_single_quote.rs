//! Credential configuration operation: shell single quote.

pub(super) fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
