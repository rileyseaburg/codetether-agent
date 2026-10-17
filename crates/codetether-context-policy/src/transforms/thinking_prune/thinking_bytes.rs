//! thinking bytes.
use super::*;

pub(super) fn thinking_bytes(p: &ContentPart) -> usize {
    match p {
        ContentPart::Thinking { text, .. } => text.len(),
        _ => 0,
    }
}
