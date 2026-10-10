mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn global_pairing_budget_counts_successes_and_resets_at_one_minute() {
    let mut registry = Registry::default();
    let first = registry.create(common::input(), common::NOW).unwrap();
    registry.pair(&first.code, common::NOW).unwrap();
    for _ in 1..30 {
        assert_eq!(
            registry.pair("000000000000", common::NOW).err(),
            Some(Error::Pairing)
        );
    }
    let second = registry.create(common::input(), common::NOW).unwrap();
    let result = registry.pair(&second.code, common::NOW + 59_999);
    assert_eq!(result.err(), Some(Error::Attempts));
    assert!(registry.pair(&second.code, common::NOW + 60_000).is_ok());
}
#[test]
fn malformed_codes_do_not_consume_registry_budget() {
    let mut registry = Registry::default();
    let receipt = registry.create(common::input(), common::NOW).unwrap();
    for _ in 0..40 {
        assert_eq!(
            registry.pair("bad", common::NOW).err(),
            Some(Error::Pairing)
        );
    }
    assert!(registry.pair(&receipt.code, common::NOW).is_ok());
}
