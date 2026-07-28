use super::cursor::Cursor;

pub(super) fn parse(cur: &mut Cursor) -> Option<String> {
    if !cur.eat('"') {
        return None;
    }
    let mut out = String::new();
    loop {
        match cur.bump_raw()? {
            '"' => return Some(out),
            '\\' => out.push(escape(cur.bump_raw()?)),
            ch => out.push(ch),
        }
    }
}

fn escape(ch: char) -> char {
    match ch {
        '"' => '"',
        '\\' => '\\',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        other => other,
    }
}
