//! Character-class matching for the JavaScript regex subset.

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let body = pattern.strip_prefix('[')?.strip_suffix(']')?;
    text.char_indices()
        .find(|(_, ch)| matches_class(body, *ch))
        .map(|(i, ch)| (i, i + ch.len_utf8()))
}

pub(super) fn matches_class(body: &str, needle: char) -> bool {
    let chars: Vec<char> = body.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let start = read_char(&chars, &mut i);
        if i + 1 < chars.len() && chars[i] == '-' {
            i += 1;
            let end = read_char(&chars, &mut i);
            if start <= needle && needle <= end {
                return true;
            }
        } else if start == needle {
            return true;
        }
    }
    false
}

fn read_char(chars: &[char], i: &mut usize) -> char {
    let ch = chars[*i];
    *i += 1;
    if ch == '\\' && *i < chars.len() {
        let escaped = chars[*i];
        *i += 1;
        escaped
    } else {
        ch
    }
}
