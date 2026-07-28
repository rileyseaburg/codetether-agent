//! Functional pseudo argument parsing.

pub(crate) fn read(source: &str, mut index: usize) -> Option<(String, usize)> {
    let mut out = String::new();
    let mut depth = 1usize;
    let mut quote = None;
    while index < source.len() {
        let ch = source[index..].chars().next()?;
        index += ch.len_utf8();
        if let Some(expected) = quote {
            if ch == expected {
                quote = None;
            }
            out.push(ch);
            continue;
        }
        match ch {
            '"' | '\'' => {
                quote = Some(ch);
                out.push(ch);
            }
            '(' => {
                depth += 1;
                out.push(ch);
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((clean(&out), index));
                }
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    None
}

fn clean(raw: &str) -> String {
    let text = raw.trim();
    match (text.strip_prefix('"'), text.strip_prefix('\'')) {
        (Some(value), _) => value.strip_suffix('"').unwrap_or(value).into(),
        (_, Some(value)) => value.strip_suffix('\'').unwrap_or(value).into(),
        _ => text.into(),
    }
}
