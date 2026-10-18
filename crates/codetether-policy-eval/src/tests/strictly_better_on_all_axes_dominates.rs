use super::fixtures::sample;

#[test]
fn strictly_better_on_all_axes_dominates() {
    let a = sample("a", 1000, 5, 2, 0.5);
    let b = sample("b", 500, 1, 0, 0.9);
    assert!(a.is_dominated_by(&b));
    assert!(!b.is_dominated_by(&a));
}
