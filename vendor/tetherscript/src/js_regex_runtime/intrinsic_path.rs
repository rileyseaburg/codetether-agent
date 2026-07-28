const PATTERN: &str = r#"[^%.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|%$))"#;

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    (pattern == PATTERN).then(|| next_part(text)).flatten()
}

fn next_part(text: &str) -> Option<(usize, usize)> {
    let start = text
        .char_indices()
        .find_map(|(index, ch)| (!matches!(ch, '%' | '.' | '[' | ']')).then_some(index))?;
    let end = text[start..]
        .char_indices()
        .find_map(|(offset, ch)| matches!(ch, '%' | '.' | '[' | ']').then_some(start + offset))
        .unwrap_or(text.len());
    Some((start, end))
}
