//! Path segment captures for route regexes.

pub(super) fn required(text: &str, index: &mut usize, captures: &mut Vec<Option<String>>) -> bool {
    let Some(capture) = read(text, *index) else {
        return false;
    };
    *index += capture.len();
    captures.push(Some(capture.into()));
    true
}

pub(super) fn optional(text: &str, index: &mut usize, captures: &mut Vec<Option<String>>) {
    if !text[*index..].starts_with('/') {
        captures.push(None);
        return;
    }
    let start = *index + 1;
    if let Some(capture) = read(text, start) {
        *index = start + capture.len();
        captures.push(Some(capture.into()));
    } else {
        captures.push(None);
    }
}

fn read(text: &str, start: usize) -> Option<&str> {
    let end = text[start..]
        .find('/')
        .map_or(text.len(), |offset| start + offset);
    (end > start).then_some(&text[start..end])
}
