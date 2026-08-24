//! Network sandbox environment synchronization for TUI startup.

/// Apply the TUI network setting to the bash sandbox environment.
pub(super) fn apply(allow_network: bool) {
    if allow_network {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1") }
    } else {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "0") }
    }
}

#[cfg(test)]
mod tests {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            unsafe {
                std::env::remove_var("CODETETHER_ALLOW_NETWORK");
                std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK");
            }
        }
    }

    #[test]
    fn tui_off_overrides_global_network_allow() {
        let _lock = crate::approval::test_env::lock_env();
        let _reset = Reset;
        unsafe { std::env::set_var("CODETETHER_ALLOW_NETWORK", "1") };
        super::apply(false);
        assert!(!crate::tool::exec_command::policy::env::network_allowed());
    }
}
