//! Isolated TypeScript workspace for approval/preflight ordering tests.

pub(super) struct Scope {
    dir: tempfile::TempDir,
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl Scope {
    pub(super) fn new() -> Self {
        let lock = crate::approval::test_env::lock_env();
        let dir = tempfile::tempdir().expect("tempdir");
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
        unsafe { std::env::set_var("XDG_CONFIG_HOME", dir.path().join("config")) };
        let config = crate::config::Config::global_config_path().expect("config path");
        std::fs::create_dir_all(config.parent().expect("config parent")).expect("config dir");
        std::fs::write(config, "approval_policy = 'on-request'").expect("config");
        std::fs::write(dir.path().join("tsconfig.json"), "{}").expect("tsconfig");
        std::fs::write(dir.path().join("broken.ts"), "export {};\n").expect("source");
        Self { dir, _lock: lock }
    }

    pub(super) fn path(&self) -> &std::path::Path {
        self.dir.path()
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
    }
}
