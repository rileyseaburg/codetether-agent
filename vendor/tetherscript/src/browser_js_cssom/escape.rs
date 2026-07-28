pub(super) fn ident(value: &str) -> String {
    let chars = value.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (index, ch) in chars.iter().copied().enumerate() {
        push_char(&mut out, &chars, index, ch);
    }
    out
}

fn push_char(out: &mut String, chars: &[char], index: usize, ch: char) {
    if ch == '\0' {
        out.push_str("\\fffd ");
    } else if needs_hex(chars, index, ch) {
        out.push_str(&format!("\\{:x} ", ch as u32));
    } else if safe_ident(chars, index, ch) {
        out.push(ch);
    } else {
        out.push('\\');
        out.push(ch);
    }
}

fn needs_hex(chars: &[char], index: usize, ch: char) -> bool {
    ch.is_ascii_control()
        || (index == 0 && ch.is_ascii_digit())
        || (index == 1 && chars.first() == Some(&'-') && ch.is_ascii_digit())
}

fn safe_ident(chars: &[char], index: usize, ch: char) -> bool {
    ch == '_'
        || ch.is_ascii_alphabetic()
        || ch as u32 >= 0x80
        || (ch.is_ascii_digit() && index > 0)
        || (ch == '-' && chars.len() > 1)
}
