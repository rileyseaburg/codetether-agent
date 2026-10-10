use super::shared::{fixture, status};
use codetether_companion_core::Registry;

#[test]
fn shared_capacity_and_pairing_window() {
    let f = fixture();
    let mut registry = Registry::default();
    for _ in 0..f.capacity {
        registry.create(f.input.clone(), f.now).unwrap();
    }
    assert_eq!(status(registry.create(f.input.clone(), f.now)), 429);
    let expires = f.now + f.session[1].age;
    assert_eq!(status(registry.create(f.input.clone(), expires)), 200);
    for _ in 0..f.attempts {
        assert_eq!(status(registry.pair("000000000000", expires)), 401);
    }
    assert_eq!(
        status(registry.pair("000000000000", expires + f.window_ms - 1)),
        429
    );
    assert_eq!(
        status(registry.pair("000000000000", expires + f.window_ms)),
        401
    );
}

#[test]
fn shared_successful_pairing_consumes_budget() {
    let f = fixture();
    let mut registry = Registry::default();
    let receipt = registry.create(f.input, f.now).unwrap();
    registry.pair(&receipt.code, f.now).unwrap();
    for _ in 1..f.attempts {
        assert_eq!(status(registry.pair("000000000000", f.now)), 401);
    }
    assert_eq!(status(registry.pair("000000000000", f.now)), 429);
}
