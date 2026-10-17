//! Recognition of delimiters following one completely decoded JSON value.

use super::super::markup;

pub(super) fn end(text: &str, json_end: usize) -> Option<usize> {
    let mut cursor = whitespace(text, json_end);
    cursor = fence(text, cursor).map_or(cursor, |end| whitespace(text, end));
    if cursor == text.len() {
        return Some(cursor);
    }
    if let Some(tag) = markup::call_close_regex().find(&text[cursor..]) {
        let tag_end = cursor + tag.end();
        let fence_start = whitespace(text, tag_end);
        return Some(fence(text, fence_start).unwrap_or(tag_end));
    }
    let tag = markup::call_open_regex().find(&text[cursor..])?;
    let end = whitespace(text, cursor + tag.end());
    (end == text.len()).then_some(end)
}

fn whitespace(text: &str, mut cursor: usize) -> usize {
    while let Some(character) = text[cursor..].chars().next() {
        if !character.is_whitespace() {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn fence(text: &str, cursor: usize) -> Option<usize> {
    text[cursor..].starts_with("```").then_some(cursor + 3)
}
