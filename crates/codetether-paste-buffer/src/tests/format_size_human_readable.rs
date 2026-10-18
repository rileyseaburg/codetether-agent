//! Paste buffer regression retained from native input handling.
use crate::*;

#[test]
fn format_size_human_readable() {
    assert_eq!(format_size(0), "0 B");
    assert_eq!(format_size(512), "512 B");
    assert_eq!(format_size(1024), "1.0 KiB");
    assert_eq!(format_size(1536), "1.5 KiB");
    assert_eq!(format_size(2 * 1024 * 1024), "2.0 MiB");
}
