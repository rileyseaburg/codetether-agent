//! Source copying helpers for regex literal rewriting.

pub(super) fn string(src: &str, b: &[u8], start: usize, out: &mut String) -> usize {
    let quote = b[start];
    let mut i = start + 1;
    while i < b.len() {
        let c = b[i];
        i += 1;
        if c == b'\\' && i < b.len() {
            i += 1;
        } else if c == quote {
            break;
        }
    }
    out.push_str(&src[start..i]);
    i
}

pub(super) fn comment(src: &str, b: &[u8], start: usize, out: &mut String) -> usize {
    let end = if b.get(start + 1) == Some(&b'/') {
        src[start..].find('\n').map_or(b.len(), |n| start + n)
    } else {
        src[start..].find("*/").map_or(b.len(), |n| start + n + 2)
    };
    out.push_str(&src[start..end]);
    end
}

pub(super) fn regex(src: &str, b: &[u8], start: usize, out: &mut String) -> usize {
    let (end, pattern) = super::regex_literal::body(src, b, start);
    let flags_end = super::regex_literal::flags(b, end);
    if out
        .as_bytes()
        .last()
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_' || *byte == b'$')
    {
        out.push(' ');
    }
    out.push_str("__regex(\"");
    out.push_str(&super::regex_literal::escape(pattern));
    out.push_str("\",\"");
    out.push_str(&src[end..flags_end]);
    out.push_str("\")");
    flags_end
}
