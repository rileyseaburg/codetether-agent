use super::shared::{fixture, status};
use codetether_companion_core::Registry;

#[test]
fn shared_expiry_boundaries() {
    let f = fixture();
    for case in &f.pairing {
        let mut registry = Registry::default();
        let receipt = registry.create(f.input.clone(), f.now).unwrap();
        assert_eq!(
            status(registry.pair(&receipt.code, f.now + case.age)),
            case.status
        );
    }
    for case in &f.session {
        let mut registry = Registry::default();
        let receipt = registry.create(f.input.clone(), f.now).unwrap();
        assert_eq!(
            status(registry.session(&receipt.id, f.now + case.age)),
            case.status
        );
    }
}
