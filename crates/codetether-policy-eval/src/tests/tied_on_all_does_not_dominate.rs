use super::fixtures::sample;

#[test]
fn tied_on_all_does_not_dominate() {
    let a = sample("a", 1000, 5, 2, 0.5);
    let b = sample("b", 1000, 5, 2, 0.5);
    assert!(!a.is_dominated_by(&b));
    assert!(!b.is_dominated_by(&a));
}
