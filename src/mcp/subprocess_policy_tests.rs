use super::{guard, policy_args};
use crate::approval::{ApprovalStore, test_env::lock_env};
use crate::config::{AccessMode, Config};

#[path = "subprocess_policy_alias_tests.rs"]
mod alias;
#[path = "subprocess_policy_approved_tests.rs"]
mod approved;
#[path = "subprocess_policy_preflight_tests.rs"]
mod preflight;
#[cfg(unix)]
#[path = "subprocess_policy_process_tests.rs"]
mod process;
#[path = "subprocess_policy_scope_tests.rs"]
mod scope;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Config::apply_process_access_mode_override(Some(AccessMode::Ask));
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        Config::apply_process_access_mode_override(None);
    }
}

#[tokio::test]
async fn subprocess_spawn_requires_policy_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    assert!(guard("npx", &["server"], None).await.is_err());
}
