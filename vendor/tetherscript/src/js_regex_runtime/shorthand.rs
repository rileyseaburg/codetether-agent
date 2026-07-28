//! Shorthand character classes for the JavaScript regex subset.

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let (kind, repeat) = match pattern {
        "\\s" => ('s', false),
        "\\s+" => ('s', true),
        "\\w" => ('w', false),
        "\\w+" => ('w', true),
        "\\d" => ('d', false),
        "\\d+" => ('d', true),
        _ => return None,
    };
    let (start, first) = text.char_indices().find(|(_, ch)| matches(kind, *ch))?;
    let mut end = start + first.len_utf8();
    if repeat {
        for (offset, ch) in text[end..].char_indices() {
            if !matches(kind, ch) {
                return Some((start, end + offset));
            }
        }
        end = text.len();
    }
    Some((start, end))
}

fn matches(kind: char, ch: char) -> bool {
    match kind {
        's' => ch.is_whitespace(),
        'w' => ch.is_ascii_alphanumeric() || ch == '_',
        'd' => ch.is_ascii_digit(),
        _ => false,
    }
}
