
use super::*;
#[test]
fn detects_split_detach_sequence() {
    let mut value = Detector::new();
    assert!(!value.filter(&[2]).detach);
    assert!(value.filter(b"d").detach);
}
#[test]
fn forwards_non_detach_prefix() {
    assert_eq!(Detector::new().filter(&[2, b'x']).data, vec![2, b'x']);
}
