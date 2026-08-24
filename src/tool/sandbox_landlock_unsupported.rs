use crate::tool::sandbox::SandboxPolicy;
use std::path::Path;

pub(crate) fn apply_std(_cmd: &mut std::process::Command, _rules: Option<Rules>) {}

#[derive(Debug, Clone)]
pub(crate) struct Rules;

#[derive(Debug)]
pub(crate) struct Prepared {
    pub rules: Option<Rules>,
    pub fallback: Option<String>,
}

pub(crate) fn prepare(_policy: &SandboxPolicy, _work_dir: &Path) -> Prepared {
    Prepared {
        rules: None,
        fallback: Some("landlock_inactive:unsupported_platform".to_string()),
    }
}

pub(crate) fn apply(_cmd: &mut tokio::process::Command, _rules: Option<Rules>) {}

pub(crate) fn abi_version() -> i64 {
    0
}