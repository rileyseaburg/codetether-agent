//! Arrow function body extraction.

pub(crate) struct ArrowBody {
    pub(crate) end: usize,
    pub(crate) text: String,
    pub(crate) block: bool,
}

pub(crate) fn find(source: &str, start: usize) -> Option<ArrowBody> {
    if source.as_bytes().get(start) == Some(&b'{') {
        let end = matching_close(source, start)? + 1;
        return Some(ArrowBody {
            end,
            text: source[start..end].into(),
            block: true,
        });
    }
    let end = expr_end(source, start);
    Some(ArrowBody {
        end,
        text: source[start..end].into(),
        block: false,
    })
}

fn matching_close(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for index in open..source.len() {
        match source.as_bytes()[index] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn expr_end(source: &str, start: usize) -> usize {
    let mut depth = 0usize;
    for index in start..source.len() {
        match source.as_bytes()[index] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' if depth == 0 => return index,
            b')' | b']' | b'}' => depth -= 1,
            b',' | b';' if depth == 0 => return index,
            _ => {}
        }
    }
    source.len()
}
