use super::{enter, leave};

#[test]
fn display_mode_prevents_bottom_row_wrapping() {
    let mut entered = Vec::new();
    enter(&mut entered).expect("enter display mode");
    assert!(entered.ends_with(b"\x1b[?7l"));

    let mut left = Vec::new();
    leave(&mut left).expect("leave display mode");
    assert!(left.windows(5).any(|bytes| bytes == b"\x1b[?7h"));
}
