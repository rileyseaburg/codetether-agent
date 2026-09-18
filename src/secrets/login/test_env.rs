//! Restore profile-directory selection even when a persistence assertion fails.

pub(super) struct Directory {
    previous: Option<std::ffi::OsString>,
}

impl Directory {
    pub(super) fn set(path: &std::path::Path) -> Self {
        let previous = std::env::var_os("CODETETHER_VAULT_CONFIG_DIR");
        // The caller holds the shared test environment lock.
        unsafe {
            std::env::set_var("CODETETHER_VAULT_CONFIG_DIR", path);
        }
        Self { previous }
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        unsafe {
            match &self.previous {
                Some(value) => std::env::set_var("CODETETHER_VAULT_CONFIG_DIR", value),
                None => std::env::remove_var("CODETETHER_VAULT_CONFIG_DIR"),
            }
        }
    }
}
