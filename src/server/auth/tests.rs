use super::{AuthState, util::constant_time_eq};
use std::sync::{Mutex, OnceLock};

static AUTH_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn constant_time_eq_works() {
    assert!(constant_time_eq(b"hello", b"hello"));
    assert!(!constant_time_eq(b"hello", b"world"));
    assert!(!constant_time_eq(b"short", b"longer"));
}

#[test]
fn auth_state_generates_token_when_env_missing() {
    let lock = AUTH_ENV_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().expect("auth env lock");
    let previous = std::env::var_os("CODETETHER_AUTH_TOKEN");
    unsafe {
        std::env::remove_var("CODETETHER_AUTH_TOKEN");
    }
    let state = AuthState::from_env();
    match previous {
        Some(value) => unsafe {
            std::env::set_var("CODETETHER_AUTH_TOKEN", value);
        },
        None => unsafe {
            std::env::remove_var("CODETETHER_AUTH_TOKEN");
        },
    }
    assert_eq!(state.token().len(), 64);
}
