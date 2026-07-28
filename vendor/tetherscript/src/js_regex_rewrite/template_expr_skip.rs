//! Skip helpers for JavaScript expressions embedded in template literals.

pub(super) fn string(b: &[u8], start: usize) -> usize {
    let quote = b[start];
    let mut i = start + 1;
    while i < b.len() {
        if b[i] == b'\\' {
            i = (i + 2).min(b.len());
        } else if b[i] == quote {
            return i + 1;
        } else {
            i += 1;
        }
    }
    i
}

pub(super) fn comment(b: &[u8], start: usize) -> usize {
    if b.get(start + 1) == Some(&b'/') {
        return b[start..]
            .iter()
            .position(|c| *c == b'\n')
            .map_or(b.len(), |n| start + n);
    }
    b[start..]
        .windows(2)
        .position(|w| w == b"*/")
        .map_or(b.len(), |n| start + n + 2)
}

pub(super) fn regex(b: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    let mut class = false;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i = (i + 2).min(b.len());
            continue;
        }
        class = c == b'[' || (class && c != b']');
        if c == b'/' && !class {
            return flags(b, i + 1);
        }
        i += 1;
    }
    i
}

fn flags(b: &[u8], mut i: usize) -> usize {
    while matches!(b.get(i), Some(b'a'..=b'z' | b'A'..=b'Z')) {
        i += 1;
    }
    i
}
