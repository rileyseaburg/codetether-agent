use crate::{RustAction, RustArgs};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Shell-free command description. Doctor's plan is its initial Cargo probe;
/// [`crate::run`] performs the remaining diagnostic probes as well.
///
/// ```
/// use codetether_rust_build::{plan, RustArgs};
/// let command = plan(&RustArgs::default()).unwrap();
/// assert_eq!(command.program, "cargo");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    /// Executable name: `cargo`, or `bash` for the repository cache pipeline.
    pub program: String,
    /// Separate argv entries, never a shell expression.
    pub args: Vec<String>,
    /// Child working directory; relative paths are relative to the caller.
    pub cwd: PathBuf,
    /// Additional argv lists for the same executable/cwd, run only after success.
    /// Cache builds attempt guarded publication after a successful build.
    #[serde(default)]
    pub after_success: Vec<Vec<String>>,
}

/// Construct a deterministic command without accessing the filesystem.
///
/// # Arguments
/// * `input` - Typed operation, selectors, directory and toolchain.
/// # Returns
/// A command plan; manifest existence is checked by [`crate::run`], not here.
/// # Errors
/// Rejects invalid selectors/toolchains and flags unsupported by an action.
/// # Examples
/// ```
/// use codetether_rust_build::{plan, RustArgs};
/// assert_eq!(plan(&RustArgs::default()).unwrap().args,
///            ["+stable", "ra-jit", "build"]);
/// ```
pub fn plan(input: &RustArgs) -> Result<Plan> {
    crate::validate::validate(input)?;
    if matches!(
        input.action,
        RustAction::CacheBuild | RustAction::CachePublish
    ) {
        return Ok(crate::cache::plan(input));
    }
    let mut args = vec![format!("+{}", input.toolchain)];
    let action = match input.action {
        RustAction::Build => "build",
        RustAction::Run => "run",
        RustAction::Test => "test",
        RustAction::Check => "check",
        RustAction::Status => "status",
        RustAction::Stop => "stop",
        RustAction::Doctor => "--version",
        RustAction::CacheBuild | RustAction::CachePublish => unreachable!("handled above"),
    };
    if !matches!(input.action, RustAction::Check | RustAction::Doctor) {
        args.push("ra-jit".into());
    }
    args.push(action.into());
    for (flag, value) in [("--package", &input.package), ("--bin", &input.bin)] {
        if let Some(value) = value {
            args.extend([flag.into(), value.clone()]);
        }
    }
    if !input.args.is_empty() {
        args.push("--".into());
        args.extend(input.args.iter().cloned());
    }
    Ok(Plan {
        program: "cargo".into(),
        args,
        cwd: input.cwd.clone(),
        after_success: Vec::new(),
    })
}
