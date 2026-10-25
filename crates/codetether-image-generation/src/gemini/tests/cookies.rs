//! Cookie export character encoding tests use synthetic values only.
use super::super::cookie_file::decode;

#[test]
fn accepts_utf8_bom_and_windows_utf16() {
    let text = ".google.com\tTRUE\t/\tTRUE\t0\ttest\tvalue\n";
    assert_eq!(decode(format!("\u{feff}{text}").as_bytes()).unwrap(), text);
    let mut bytes = vec![0xff, 0xfe];
    bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
    assert_eq!(decode(&bytes).unwrap(), text);
}

#[test]
fn rejects_broken_and_oversized_exports() {
    assert!(decode(&[0xff, 0xfe, 1]).is_err());
    assert!(decode(&vec![0; 1024 * 1024 + 1]).is_err());
}
