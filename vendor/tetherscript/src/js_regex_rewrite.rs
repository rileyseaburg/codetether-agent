//! Rewrites JavaScript regex literals into runtime helper calls.

#[path = "js_regex_rewrite/context.rs"]
mod context;
#[path = "js_regex_rewrite/copy.rs"]
mod copy;
#[path = "js_regex_rewrite/regex_literal.rs"]
mod regex_literal;
#[path = "js_regex_rewrite/template.rs"]
mod template;
#[path = "js_regex_rewrite/template_expr_skip.rs"]
mod template_expr_skip;
#[path = "js_regex_rewrite/template_scan.rs"]
mod template_scan;
#[path = "js_regex_rewrite/template_skip.rs"]
mod template_skip;
#[path = "js_regex_rewrite/template_text.rs"]
mod template_text;

pub(crate) fn rewrite(src: &str) -> String {
    let b = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let mut prev = b'(';
    while i < b.len() {
        let c = b[i];
        if c == b'`' {
            i = template::rewrite(src, b, i, &mut out);
            prev = b')';
            continue;
        } else if matches!(c, b'\'' | b'"') {
            i = copy::string(src, b, i, &mut out);
        } else if c == b'/' && matches!(b.get(i + 1), Some(b'/' | b'*')) {
            i = copy::comment(src, b, i, &mut out);
        } else if c == b'/' && context::regex(prev, b, i) {
            i = copy::regex(src, b, i, &mut out);
            prev = b')';
            continue;
        } else {
            out.push(c as char);
            i += 1;
        }
        if !c.is_ascii_whitespace() {
            prev = c;
        }
    }
    out
}
