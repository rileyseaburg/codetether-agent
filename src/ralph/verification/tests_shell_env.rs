use crate::config::ProjectTrustStore;
use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};

pub(super) struct TrustedShellProject {
    _data: TempDir,
    _cwd: CwdGuard,
}

impl TrustedShellProject {
    pub(super) fn new(path: &Path) -> Self {
        let data = tempdir().expect("data dir");
        unsafe {
            std::env::set_var("CODETETHER_DATA_DIR", data.path());
            std::env::set_var("CODETETHER_UNSANDBOXED_BASH", "1");
        }
        let cwd = CwdGuard::enter(path);
        ProjectTrustStore::for_current_workspace()
            .expect("trust store")
            .trust()
            .expect("trust workspace");
        std::fs::write("codetether.toml", "approval_policy = \"never\"\n").expect("config");
        Self {
            _data: data,
            _cwd: cwd,
        }
    }
}

impl Drop for TrustedShellProject {
    fn drop(&mut self) {
        unsafe {
            std::env::remove_var("CODETETHER_DATA_DIR");
            std::env::remove_var("CODETETHER_UNSANDBOXED_BASH");
        }
    }
}

struct CwdGuard(PathBuf);

impl CwdGuard {
    fn enter(path: &Path) -> Self {
        let previous = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(path).expect("set cwd");
        Self(previous)
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.0).expect("restore cwd");
    }
}
