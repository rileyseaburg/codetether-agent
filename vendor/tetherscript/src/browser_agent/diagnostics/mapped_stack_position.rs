//! Generated JavaScript position helpers.

pub fn index(source: &str, line: usize, column: usize) -> Option<usize> {
    let mut offset = 0usize;
    for (row, text) in source.split_inclusive('\n').enumerate() {
        if row + 1 == line {
            return Some(offset + column.checked_sub(1)?);
        }
        offset += text.len();
    }
    None
}

pub fn line_column(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;
    for (offset, ch) in source.char_indices() {
        if offset >= index {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

pub fn call(source: &str, start: usize, end: usize, name: &str) -> Option<usize> {
    source[start..end]
        .find(&format!("{name}("))
        .map(|offset| start + offset)
}
