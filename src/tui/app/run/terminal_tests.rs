#![cfg(unix)]

use super::clear_screen;

#[test]
fn startup_clear_does_not_query_cursor_position() {
    let mut output = Vec::new();

    clear_screen(&mut output).expect("screen clear should be writable");

    assert_eq!(output, b"\x1b[2J");
    assert!(!output.windows(3).any(|bytes| bytes == b"[6n"));
}
