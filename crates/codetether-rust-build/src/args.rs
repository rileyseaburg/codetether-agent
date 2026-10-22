use crate::RustAction;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Arguments shared by the first-party CLI and programmatic callers.
///
/// ```
/// use codetether_rust_build::RustArgs;
/// assert_eq!(RustArgs::default().toolchain, "stable");
/// ```
#[derive(Args, Clone, Debug, Serialize, Deserialize)]
pub struct RustArgs {
    /// Explicit development operation.
    #[arg(value_enum)]
    pub action: RustAction,
    /// Directory containing Cargo.toml for build/run/test/check.
    #[arg(long, default_value = ".")]
    pub cwd: PathBuf,
    /// Rustup toolchain; ra-jit requires its rustc to be exactly 1.99.0.
    #[arg(long, default_value = "stable")]
    pub toolchain: String,
    /// One package name (not a Cargo package-spec expression).
    #[arg(long)]
    pub package: Option<String>,
    /// One binary target name.
    #[arg(long)]
    pub bin: Option<String>,
    /// Literal program/test argument; repeat --arg=VALUE. Only run/test accept it.
    #[arg(long = "arg", allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl Default for RustArgs {
    fn default() -> Self {
        Self {
            action: RustAction::Build,
            cwd: ".".into(),
            toolchain: "stable".into(),
            package: None,
            bin: None,
            args: Vec::new(),
        }
    }
}
