pub(crate) fn comment(
    b: &[u8],
    start: usize,
    mut line: usize,
    mut col: usize,
) -> (usize, usize, usize) {
    let block = b.get(start + 1) == Some(&b'*');
    let mut i = start + 2;
    col += 2;
    while i < b.len() {
        if !block && b[i] == b'\n' {
            break;
        }
        if block && b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
            return (i + 2, line, col + 2);
        }
        if b[i] == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        i += 1;
    }
    (i, line, col)
}
