//! Lightweight symbol extraction for code-shaped text.

use super::EvidenceKind;

/// Extract a likely symbol name from a source line.
pub(super) fn name(line: &str) -> Option<String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if is_decl(word) {
            return words.get(i + 1).and_then(|w| ident(w));
        }
    }
    None
}

/// Classify a line into an evidence kind.
pub(super) fn kind(line: &str) -> EvidenceKind {
    let lower = line.to_lowercase();
    if name(line).is_some() {
        EvidenceKind::Symbol
    } else if lower.trim_start().starts_with("use ") || lower.contains(" import ") {
        EvidenceKind::Import
    } else if lower.contains("error")
        || lower.contains("panic")
        || lower.contains("unwrap")
        || lower.contains("failed")
    {
        EvidenceKind::Error
    } else {
        EvidenceKind::Text
    }
}

fn is_decl(word: &str) -> bool {
    matches!(
        word.trim_matches(|c: char| !c.is_alphanumeric()),
        "fn" | "struct" | "enum" | "trait" | "mod" | "impl"
    )
}

fn ident(word: &&str) -> Option<String> {
    let value = word.trim_matches(|c: char| !(c.is_alphanumeric() || c == '_'));
    (!value.is_empty()).then(|| value.to_string())
}
