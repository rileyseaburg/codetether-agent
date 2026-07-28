//! Arrow function parameter extraction.

pub(crate) struct ArrowParams {
    pub(crate) start: usize,
    pub(crate) text: String,
}

pub(crate) fn find(source: &str, arrow: usize) -> Option<ArrowParams> {
    let end = trim_left(source, arrow);
    if source.as_bytes().get(end.checked_sub(1)?) == Some(&b')') {
        return paren_params(source, end - 1);
    }
    ident_params(source, end)
}

fn paren_params(source: &str, close: usize) -> Option<ArrowParams> {
    let open = matching_open(source, close)?;
    Some(ArrowParams {
        start: open,
        text: source[open + 1..close].trim().into(),
    })
}

fn ident_params(source: &str, end: usize) -> Option<ArrowParams> {
    let mut start = end;
    while start > 0 {
        let byte = source.as_bytes()[start - 1];
        if !(byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$') {
            break;
        }
        start -= 1;
    }
    (start < end).then(|| ArrowParams {
        start,
        text: source[start..end].trim().into(),
    })
}

fn matching_open(source: &str, close: usize) -> Option<usize> {
    let mut depth = 0usize;
    for index in (0..close).rev() {
        match source.as_bytes()[index] {
            b')' => depth += 1,
            b'(' if depth == 0 => return Some(index),
            b'(' => depth -= 1,
            _ => {}
        }
    }
    None
}

fn trim_left(source: &str, mut index: usize) -> usize {
    while index > 0 && source.as_bytes()[index - 1].is_ascii_whitespace() {
        index -= 1;
    }
    index
}
