mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn interval_and_timestamp_boundaries() {
    for (interval, accepted) in [(14, false), (15, true), (300, true), (301, false)] {
        let mut input = common::input();
        input.interval_seconds = interval;
        assert_eq!(
            Registry::default().create(input, common::NOW).is_ok(),
            accepted
        );
    }
    let mut registry = Registry::default();
    assert_eq!(
        registry.create(common::input(), i64::MAX).err(),
        Some(Error::Configuration)
    );
    assert!(registry.is_empty());
}
