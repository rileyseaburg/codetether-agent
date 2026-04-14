use super::{AuthState, util::constant_time_eq};

#[test]
fn constant_time_eq_works() {
    assert!(constant_time_eq(b"hello", b"hello"));
    assert!(!constant_time_eq(b"hello", b"world"));
    assert!(!constant_time_eq(b"short", b"longer"));
}

#[test]
fn auth_state_generates_token_when_env_missing() {
    unsafe {
        std::env::remove_var("CODETETHER_AUTH_TOKEN");
    }
    let state = AuthState::from_env();
    assert_eq!(state.token().len(), 64);
}
