//! Flexbox unit tests.

use super::perform_flex_layout;
use std::collections::HashMap;

fn map(p: &[(&str, &str)]) -> HashMap<String, String> {
    p.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn row_layout_places_items() {
    let c = map(&[("display", "flex")]);
    let kids = vec![map(&[]), map(&[])];
    let r = perform_flex_layout(&c, 100, &kids, &[(20, 10), (30, 10)]);
    assert_eq!((r[0].x, r[1].x), (0, 20));
}

#[test]
fn column_layout_uses_y_axis() {
    let c = map(&[("flex-direction", "column"), ("height", "100")]);
    let kids = vec![map(&[]), map(&[])];
    let r = perform_flex_layout(&c, 50, &kids, &[(10, 20), (10, 30)]);
    assert_eq!((r[0].y, r[1].y), (0, 20));
}

#[test]
fn flex_grow_distributes_space() {
    let c = map(&[]);
    let grow = vec![map(&[("flex-grow", "1")]), map(&[("flex-grow", "1")])];
    let r = perform_flex_layout(&c, 100, &grow, &[(20, 10), (20, 10)]);
    assert_eq!((r[0].width, r[1].width), (50, 50));
}

#[test]
fn justify_variants() {
    for (jc, x0) in [("flex-end", 60), ("center", 30), ("space-evenly", 20)] {
        let c = map(&[("justify-content", jc)]);
        let kids = vec![map(&[]), map(&[])];
        let r = perform_flex_layout(&c, 100, &kids, &[(20, 10), (20, 10)]);
        assert_eq!(r[0].x, x0, "failed for justify-content={}", jc);
    }
}

#[test]
fn wrap_creates_second_line() {
    let c = map(&[("flex-wrap", "wrap")]);
    let kids = vec![map(&[]), map(&[]), map(&[])];
    let r = perform_flex_layout(&c, 50, &kids, &[(30, 10), (30, 10), (10, 10)]);
    assert_eq!((r[0].y, r[1].y, r[2].y), (0, 10, 10));
}
