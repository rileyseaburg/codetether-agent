//! Escaped literal matching for the JavaScript regex subset.

pub(super) fn repeat(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let (ch, rest) = escaped_prefix(pattern)?;
    if rest != "+" {
        return None;
    }
    let start = text.find(ch)?;
    let end = text[start..]
        .find(|candidate| candidate != ch)
        .map_or(text.len(), |i| start + i);
    Some((start, end))
}

pub(super) fn single(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let (ch, rest) = escaped_prefix(pattern)?;
    if !rest.is_empty() {
        return None;
    }
    text.find(ch).map(|i| (i, i + ch.len_utf8()))
}

fn escaped_prefix(pattern: &str) -> Option<(char, &str)> {
    let mut chars = pattern.chars();
    (chars.next()? == '\\').then_some(())?;
    let ch = decode(chars.next()?);
    Some((ch, chars.as_str()))
}

fn decode(ch: char) -> char {
    match ch {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        other => other,
    }
}
