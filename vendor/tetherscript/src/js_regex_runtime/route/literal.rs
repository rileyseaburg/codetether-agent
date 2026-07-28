//! Literal token matching for route regexes.

pub(super) fn read(pattern: &str) -> (char, &str) {
    let mut chars = pattern.chars();
    let first = chars.next().unwrap();
    if first == '\\' {
        (chars.next().unwrap_or(first), chars.as_str())
    } else {
        (first, chars.as_str())
    }
}

pub(super) fn take(text: &str, index: &mut usize, expected: char, insensitive: bool) -> bool {
    let Some(found) = text[*index..].chars().next() else {
        return false;
    };
    let matched = if insensitive {
        found.eq_ignore_ascii_case(&expected)
    } else {
        found == expected
    };
    *index += found.len_utf8();
    matched
}
