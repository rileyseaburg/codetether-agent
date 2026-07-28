//! Regex literal context classification.

pub(super) fn regex(previous: u8, b: &[u8], i: usize) -> bool {
    matches!(
        previous,
        b'(' | b'=' | b':' | b',' | b'[' | b'{' | b'!' | b'?' | b';' | b'>' | b'|' | b'&'
    ) || keyword(b, i)
}

fn keyword(b: &[u8], i: usize) -> bool {
    let mut end = i;
    while end > 0 && b[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    let mut start = end;
    while start > 0 && ident_byte(b[start - 1]) {
        start -= 1;
    }
    matches!(
        &b[start..end],
        b"return" | b"throw" | b"case" | b"yield" | b"await" | b"typeof" | b"delete" | b"void"
    )
}

fn ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}
