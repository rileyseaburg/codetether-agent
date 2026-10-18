//! Network sandbox environment synchronization for TUI startup.

/// Apply the TUI network setting to the bash sandbox environment.
pub(super) fn apply(allow_network: bool) {
    if allow_network {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1") }
    } else {
        unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") }
    }
}
