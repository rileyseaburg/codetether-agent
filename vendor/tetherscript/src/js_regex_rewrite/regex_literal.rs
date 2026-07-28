//! Regex literal body, flags, and escaping helpers.

pub(super) fn body<'a>(src: &'a str, b: &[u8], start: usize) -> (usize, &'a str) {
    let mut i = start + 1;
    let mut class = false;
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i += 2;
            continue;
        }
        class = (c == b'[') || (class && c != b']');
        if c == b'/' && !class {
            return (i + 1, &src[start + 1..i]);
        }
        i += 1;
    }
    (start + 1, "")
}

pub(super) fn flags(b: &[u8], mut i: usize) -> usize {
    while matches!(b.get(i), Some(b'a'..=b'z' | b'A'..=b'Z')) {
        i += 1;
    }
    i
}

pub(super) fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
