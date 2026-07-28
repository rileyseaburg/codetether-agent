//! Template literal lowering for the source rewrite pass.

pub(super) fn rewrite(src: &str, b: &[u8], start: usize, out: &mut String) -> usize {
    let mut i = start + 1;
    let mut text_start = i;
    let mut parts = 0;
    let mark = out.len();
    out.push('(');
    while i < b.len() {
        match b[i] {
            b'\\' => i = (i + 2).min(b.len()),
            b'`' => {
                super::template_text::literal(out, &src[text_start..i], &mut parts);
                i += 1;
                break;
            }
            b'$' if b.get(i + 1) == Some(&b'{') => {
                super::template_text::literal(out, &src[text_start..i], &mut parts);
                let expr_start = i + 2;
                let expr_end = super::template_scan::expr_end(src.as_bytes(), expr_start);
                let expr = super::rewrite(&src[expr_start..expr_end]);
                super::template_text::expr(out, &expr, &mut parts);
                i = expr_end.saturating_add(1).min(b.len());
                text_start = i;
            }
            _ => i += 1,
        }
    }
    if parts == 0 {
        out.truncate(mark);
        out.push_str("\"\"");
    } else {
        out.push(')');
    }
    i
}
