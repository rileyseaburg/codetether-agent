mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn capacity_sweeps_stopped_and_expired_sessions() {
    let mut registry = Registry::default();
    let first = registry.create(common::input(), common::NOW).unwrap();
    for _ in 0..3 {
        registry.create(common::input(), common::NOW).unwrap();
    }
    assert_eq!(
        registry.create(common::input(), common::NOW).err(),
        Some(Error::Capacity)
    );
    registry.stop(&first.id, common::NOW).unwrap();
    let second = registry.create(common::input(), common::NOW).unwrap();
    assert_ne!(first.id, second.id);
    assert_ne!(first.code, second.code);
    assert_eq!(
        uuid::Uuid::parse_str(&first.id).unwrap().get_version_num(),
        4
    );
    registry
        .create(common::input(), common::NOW + 3_600_000)
        .unwrap();
    assert_eq!(registry.len(), 1);
    registry.sweep(common::NOW + 7_200_000);
    assert!(registry.is_empty());
}
