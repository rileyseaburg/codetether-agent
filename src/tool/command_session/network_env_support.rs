//! Scoped network-policy environment for command tests.

pub(super) struct DisabledNetwork(Option<std::ffi::OsString>);

impl DisabledNetwork {
    pub(super) fn set() -> Self {
        let previous = std::env::var_os("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK");
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "0") };
        Self(previous)
    }

    pub(super) fn allow(value: bool) {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", value.to_string()) };
    }
}

impl Drop for DisabledNetwork {
    fn drop(&mut self) {
        match self.0.take() {
            Some(value) => unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", value) },
            None => unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") },
        }
    }
}