//! Generated JavaScript function range extraction.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionRange {
    pub name: String,
    pub body_start: usize,
    pub body_end: usize,
}

pub fn collect(source: &str) -> Vec<FunctionRange> {
    let mut out = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find("function ") {
        let start = cursor + offset + "function ".len();
        let Some((name, open)) = name_and_open(source, start) else {
            cursor = start;
            continue;
        };
        if let Some(close) = super::mapped_stack_brace::matching(source, open) {
            out.push(FunctionRange {
                name,
                body_start: open + 1,
                body_end: close,
            });
            cursor = close + 1;
        } else {
            break;
        }
    }
    out
}

fn name_and_open(source: &str, start: usize) -> Option<(String, usize)> {
    let tail = &source[start..];
    let name_len = tail
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .map(char::len_utf8)
        .sum::<usize>();
    let name = source[start..start + name_len].to_string();
    let open = source[start + name_len..].find('{')? + start + name_len;
    Some((name, open))
}
