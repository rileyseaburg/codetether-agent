//! Adapter to the repository-owned pipeline; never reimplements cache keys or receipts.
use crate::{Plan, RustAction, RustArgs};
use anyhow::{Context, Result, ensure};
use std::path::Path;

const SCRIPT: &str = "scripts/dev/release-build-loop.sh";

pub(crate) fn plan(input: &RustArgs) -> Plan {
    let build = input.action == RustAction::CacheBuild;
    Plan {
        program: "bash".into(),
        args: vec![
            SCRIPT.into(),
            if build { "build" } else { "publish" }.into(),
        ],
        cwd: input.cwd.clone(),
        after_success: if build {
            vec![vec![SCRIPT.into(), "publish".into()]]
        } else {
            Vec::new()
        },
    }
}

pub(crate) fn validate_workspace(cwd: &Path) -> Result<()> {
    ensure!(
        cwd.join("Cargo.toml").is_file(),
        "Cargo.toml required in {}",
        cwd.display()
    );
    let root = cwd.canonicalize().context("workspace does not exist")?;
    let script = root.join(SCRIPT).canonicalize().with_context(|| {
        format!("repository cache pipeline missing: {SCRIPT}; cache actions require the repository-owned build/publish contract")
    })?;
    ensure!(
        script.is_file() && script.starts_with(&root),
        "cache pipeline must be a file inside the selected workspace"
    );
    Ok(())
}
