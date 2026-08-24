//! Scoped trusted-network setting for policy tests.

pub(crate) struct Network;

impl Network {
    pub(crate) fn set(value: &str) -> Self {
        Self::update(value);
        Self
    }

    pub(crate) fn update(value: &str) {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", value) };
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") };
    }
}