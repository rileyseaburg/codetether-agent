use super::{delta, merge, outcome::Outcome};
use std::collections::BTreeSet;

mod fixture;
use fixture::Fixture;

#[test]
fn excludes_patch_equivalent_history_and_merges_only_feature_files() {
    let fixture = Fixture::new();
    let expected = (1..=6)
        .map(|n| format!("live-card-{n}.ts"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        delta::expected(fixture.root(), "feature").unwrap(),
        expected
    );
    let Outcome::Merged(actual) = merge::merge(fixture.root(), "feature").unwrap() else {
        panic!("fixture should merge without conflicts");
    };
    assert_eq!(actual, expected);
    assert_eq!(delta::actual(fixture.root()).unwrap(), expected);
}
