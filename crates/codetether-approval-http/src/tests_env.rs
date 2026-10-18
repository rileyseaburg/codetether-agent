//! Scoped data directory and grant cleanup for route tests.
pub(super) struct EnvGuard;

impl EnvGuard {
    pub(super) fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        codetether_approval_core::session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}
