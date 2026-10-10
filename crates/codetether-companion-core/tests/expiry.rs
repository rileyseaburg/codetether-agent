mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn pairing_and_session_expiry_use_inclusive_deadlines() {
    for (elapsed, accepted) in [(299_999, true), (300_000, false)] {
        let mut registry = Registry::default();
        let receipt = registry.create(common::input(), common::NOW).unwrap();
        let result = registry.pair(&receipt.code, common::NOW + elapsed);
        assert_eq!(result.is_ok(), accepted);
    }
    let mut registry = Registry::default();
    let receipt = registry.create(common::input(), common::NOW).unwrap();
    let pair = registry.pair(&receipt.code, common::NOW).unwrap();
    let auth = format!("Bearer {}", pair.device_token);
    let before = registry.authorize_device(&receipt.id, Some(&auth), common::NOW + 3_599_999);
    assert!(before.is_ok());
    let deadline = registry.authorize_device(&receipt.id, Some(&auth), common::NOW + 3_600_000);
    assert_eq!(deadline, Err(Error::Ended));
    let mut restarted = Registry::default();
    assert!(
        restarted
            .authorize_device(&receipt.id, Some(&auth), common::NOW)
            .is_err()
    );
}
