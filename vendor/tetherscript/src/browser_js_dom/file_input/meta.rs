use super::*;

pub(super) fn parse(input: &str) -> Vec<AgentFile> {
    let mut cur = cursor::Cursor::new(input);
    let mut files = Vec::new();
    if !cur.eat('[') {
        return files;
    }
    loop {
        if cur.eat(']') {
            return files;
        }
        let Some(file) = object(&mut cur) else {
            return Vec::new();
        };
        files.push(file);
        if cur.eat(']') {
            return files;
        }
        if !cur.eat(',') {
            return Vec::new();
        }
    }
}

fn object(cur: &mut cursor::Cursor) -> Option<AgentFile> {
    if !cur.eat('{') {
        return None;
    }
    let mut file = AgentFile::default();
    loop {
        if cur.eat('}') {
            return Some(file);
        }
        let key = json_string::parse(cur)?;
        if !cur.eat(':') {
            return None;
        }
        meta_field::apply(&mut file, &key, cur)?;
        if cur.eat('}') {
            return Some(file);
        }
        if !cur.eat(',') {
            return None;
        }
    }
}
