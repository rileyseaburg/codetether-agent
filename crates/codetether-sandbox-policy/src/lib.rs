//! Sandbox policy records and platform-independent pre-spawn validation.
//!
//! [SandboxPolicy] controls allowed paths, execution, networking and budgets.
//! Validate a working directory and command arguments before launching a tool;
//! [SandboxResult] records execution output and confinement evidence.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
mod network;
mod paths;
mod policy;
mod result;
pub use network::validate as validate_network;
pub use paths::{validate_command_args, validate_working_dir};
pub use policy::SandboxPolicy;
pub use result::SandboxResult;
