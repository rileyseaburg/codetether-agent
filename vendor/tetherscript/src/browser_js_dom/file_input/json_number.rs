use super::cursor::Cursor;

pub(super) fn parse(cur: &mut Cursor) -> Option<f64> {
    cur.skip_ws();
    let mut raw = String::new();
    if cur.peek_raw() == Some('-') {
        raw.push(cur.bump_raw()?);
    }
    take_digits(cur, &mut raw);
    if cur.peek_raw() == Some('.') {
        raw.push(cur.bump_raw()?);
        take_digits(cur, &mut raw);
    }
    if raw.is_empty() || raw == "-" {
        return None;
    }
    raw.parse().ok()
}

fn take_digits(cur: &mut Cursor, out: &mut String) {
    while cur.peek_raw().is_some_and(|ch| ch.is_ascii_digit()) {
        if let Some(ch) = cur.bump_raw() {
            out.push(ch);
        }
    }
}
