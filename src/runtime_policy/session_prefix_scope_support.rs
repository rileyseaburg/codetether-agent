//! Isolated approval environment for command-prefix scope tests.

use crate::approval::test_env::{ScopedEnv, lock_env};

pub(super) struct Scope {
    _env: ScopedEnv,
    _data: tempfile::TempDir,
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl Scope {
    pub(super) fn new() -> Self {
        let lock = lock_env();
        let data = tempfile::tempdir().expect("tempdir");
        let env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
        Self {
            _env: env,
            _data: data,
            _lock: lock,
        }
    }
}
