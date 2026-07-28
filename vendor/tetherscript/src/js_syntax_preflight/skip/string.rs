pub(crate) fn string(
    b: &[u8],
    start: usize,
    mut line: usize,
    mut col: usize,
) -> (usize, usize, usize) {
    let quote = b[start];
    let mut i = start + 1;
    col += 1;
    while i < b.len() {
        let byte = b[i];
        i += 1;
        if byte == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        if byte == b'\\' && i < b.len() {
            i += 1;
            col += 1;
        } else if byte == quote {
            break;
        }
    }
    (i, line, col)
}
