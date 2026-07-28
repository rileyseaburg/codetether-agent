//! Output helpers for template literal lowering.

pub(super) fn literal(out: &mut String, text: &str, parts: &mut usize) {
    if text.is_empty() {
        return;
    }
    join(out, parts);
    out.push('"');
    for ch in text.chars() {
        if ch == '"' {
            out.push_str("\\\"");
        } else {
            out.push(ch);
        }
    }
    out.push('"');
}

pub(super) fn expr(out: &mut String, expr: &str, parts: &mut usize) {
    join(out, parts);
    out.push('(');
    out.push_str(expr);
    out.push(')');
}

fn join(out: &mut String, parts: &mut usize) {
    if *parts > 0 {
        out.push('+');
    }
    *parts += 1;
}
