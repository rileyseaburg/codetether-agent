//! Correlated relay first text part.
use super::*;

pub(super) fn first_text_part(parts: &[Part]) -> Option<String> {
    parts.iter().find_map(|part| match part {
        Part::Text { text } => Some(text.clone()),
        _ => None,
    })
}
