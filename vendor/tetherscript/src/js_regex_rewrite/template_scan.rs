//! Balanced scanners used while lowering template literal expressions.

pub(super) fn expr_end(b: &[u8], start: usize) -> usize {
    let mut i = start;
    let mut depth = 0;
    let mut prev = b'(';
    while i < b.len() {
        let c = b[i];
        if matches!(c, b'\'' | b'"') {
            i = super::template_expr_skip::string(b, i);
            prev = b'"';
            continue;
        }
        if c == b'`' {
            i = super::template_skip::template(b, i);
            prev = b')';
            continue;
        }
        if c == b'/' && matches!(b.get(i + 1), Some(b'/' | b'*')) {
            i = super::template_expr_skip::comment(b, i);
            continue;
        }
        if c == b'/' && super::context::regex(prev, b, i) {
            i = super::template_expr_skip::regex(b, i);
            prev = b')';
            continue;
        }
        match c {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' if depth == 0 => return i,
            b'}' => {
                depth -= 1;
                i += 1;
            }
            _ => i += 1,
        }
        if !c.is_ascii_whitespace() {
            prev = c;
        }
    }
    b.len()
}
