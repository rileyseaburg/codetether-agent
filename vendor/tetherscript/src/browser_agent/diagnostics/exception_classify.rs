//! Runtime exception message classification.

use super::exception_kind::RuntimeExceptionKind;

pub fn kind(message: &str) -> RuntimeExceptionKind {
    let lower = message.to_ascii_lowercase();
    if message.starts_with("ReferenceError:") {
        RuntimeExceptionKind::Reference
    } else if message.starts_with("TypeError:") {
        RuntimeExceptionKind::Type
    } else if syntax(message) {
        RuntimeExceptionKind::Syntax
    } else if permission(&lower) {
        RuntimeExceptionKind::Permission
    } else if network(&lower) {
        RuntimeExceptionKind::Network
    } else {
        RuntimeExceptionKind::Other
    }
}

fn syntax(message: &str) -> bool {
    message.starts_with("SyntaxError:")
        || message.starts_with("Unexpected character")
        || message.starts_with("Unterminated string")
        || message.starts_with("Invalid number")
        || message.starts_with("Expected ")
}

fn permission(lower: &str) -> bool {
    lower.contains("notallowederror")
        || lower.contains("permission denied")
        || lower.contains(" denied")
}

fn network(lower: &str) -> bool {
    lower.contains("aborterror")
        || lower.contains("cors blocked")
        || lower.contains("network")
        || lower.contains("offline")
}
